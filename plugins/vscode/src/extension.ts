import * as path from "path";
import * as fs from "fs";
import * as os from "os";
import { ExtensionContext, workspace, window } from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

/** Convert a Windows path (`C:\Users\...`) to a WSL path (`/mnt/c/Users/...`). */
function windowsToWslPath(winPath: string): string {
  const match = winPath.match(/^([A-Za-z]):[\\/](.*)$/);
  if (!match) {
    return winPath.replace(/\\/g, "/");
  }
  const drive = match[1].toLowerCase();
  const rest = match[2].replace(/\\/g, "/");
  return `/mnt/${drive}/${rest}`;
}

/** Normalize to an absolute path; optionally map to WSL form for `wsl.exe -e`. */
function toServerPath(fsPath: string, useWsl: boolean): string {
  const absolute = path.isAbsolute(fsPath) ? fsPath : path.resolve(fsPath);
  return useWsl ? windowsToWslPath(absolute) : absolute;
}

function fileExists(fsPath: string): boolean {
  try {
    return fs.existsSync(fsPath) && fs.statSync(fsPath).isFile();
  } catch {
    return false;
  }
}

/**
 * Candidate locations for `raptor-lsp`, in priority order.
 * Does not depend on the process cwd alone — walks workspace folders and the
 * extension install path (dev layout: plugins/vscode → tools/lsp/...).
 */
function candidateServerBinaries(context: ExtensionContext): string[] {
  const names =
    os.platform() === "win32"
      ? ["raptor-lsp.exe", "raptor-lsp"]
      : ["raptor-lsp"];

  const dirs: string[] = [];

  for (const folder of workspace.workspaceFolders ?? []) {
    const root = folder.uri.fsPath;
    dirs.push(
      path.join(root, "tools", "lsp", "target", "release"),
      path.join(root, "tools", "lsp", "target", "debug"),
      path.join(root, "target", "release"),
      path.join(root, "target", "debug"),
    );
    // Opened a subdirectory of the repo: walk a few parents.
    let parent = root;
    for (let i = 0; i < 5; i++) {
      const next = path.dirname(parent);
      if (next === parent) {
        break;
      }
      parent = next;
      dirs.push(
        path.join(parent, "tools", "lsp", "target", "release"),
        path.join(parent, "tools", "lsp", "target", "debug"),
      );
    }
  }

  // Extension shipped from the monorepo: plugins/vscode → ../../tools/lsp/...
  dirs.push(
    context.asAbsolutePath(
      path.join("..", "..", "tools", "lsp", "target", "release"),
    ),
    context.asAbsolutePath(
      path.join("..", "..", "tools", "lsp", "target", "debug"),
    ),
    // Legacy layout next to the extension folder
    context.asAbsolutePath(path.join("..", "target", "release")),
  );

  const out: string[] = [];
  const seen = new Set<string>();
  for (const dir of dirs) {
    for (const name of names) {
      const full = path.join(dir, name);
      if (!seen.has(full)) {
        seen.add(full);
        out.push(full);
      }
    }
  }
  return out;
}

function candidateBackendBinaries(): string[] {
  const names =
    os.platform() === "win32"
      ? ["lsp-backend.exe", "lsp-backend"]
      : ["lsp-backend"];

  const dirs: string[] = [];
  for (const folder of workspace.workspaceFolders ?? []) {
    const root = folder.uri.fsPath;
    dirs.push(path.join(root, "build"), root);
    let parent = root;
    for (let i = 0; i < 5; i++) {
      const next = path.dirname(parent);
      if (next === parent) {
        break;
      }
      parent = next;
      dirs.push(path.join(parent, "build"));
    }
  }

  const out: string[] = [];
  const seen = new Set<string>();
  for (const dir of dirs) {
    for (const name of names) {
      const full = path.join(dir, name);
      if (!seen.has(full)) {
        seen.add(full);
        out.push(full);
      }
    }
  }
  return out;
}

function resolveServerCommand(context: ExtensionContext): {
  command: string;
  args: string[];
  /** Absolute host path used for existsSync checks (never wsl.exe). */
  resolvedHostPath: string | undefined;
} {
  // settings.json:
  //   "raptor.serverPath": absolute path to raptor-lsp (Linux path if useWsl)
  //   "raptor.useWsl": true/false
  const config = workspace.getConfiguration("raptor");
  const configuredPath = (config.get<string>("serverPath") ?? "").trim();
  const useWsl = config.get<boolean>("useWsl", os.platform() === "win32");

  let hostPath: string | undefined;

  if (configuredPath.length > 0) {
    // User override: allow relative to first workspace folder.
    if (!path.isAbsolute(configuredPath) && workspace.workspaceFolders?.[0]) {
      hostPath = path.join(
        workspace.workspaceFolders[0].uri.fsPath,
        configuredPath,
      );
    } else {
      hostPath = configuredPath;
    }
  } else {
    hostPath = candidateServerBinaries(context).find(fileExists);
  }

  if (!hostPath) {
    return {
      command: useWsl ? "wsl.exe" : "raptor-lsp",
      args: useWsl ? ["-e", "raptor-lsp"] : [],
      resolvedHostPath: undefined,
    };
  }

  const serverPath = toServerPath(hostPath, useWsl);
  if (useWsl) {
    return { command: "wsl.exe", args: ["-e", serverPath], resolvedHostPath: hostPath };
  }
  return { command: serverPath, args: [], resolvedHostPath: hostPath };
}

function resolveBackendEnv(): NodeJS.ProcessEnv {
  const config = workspace.getConfiguration("raptor");
  const useWsl = config.get<boolean>("useWsl", os.platform() === "win32");
  const configured = (config.get<string>("backendPath") ?? "").trim();

  const env: NodeJS.ProcessEnv = { ...process.env };

  let hostBackend: string | undefined;
  if (configured.length > 0) {
    if (!path.isAbsolute(configured) && workspace.workspaceFolders?.[0]) {
      hostBackend = path.join(
        workspace.workspaceFolders[0].uri.fsPath,
        configured,
      );
    } else {
      hostBackend = configured;
    }
  } else {
    hostBackend = candidateBackendBinaries().find(fileExists);
  }

  if (hostBackend) {
    // Path as seen *inside* the server process (Linux when useWsl).
    const backendForServer = toServerPath(hostBackend, useWsl);
    env.RAPTOR_LSP_BACKEND = backendForServer;

    // wsl.exe does not forward arbitrary Windows env into Linux unless listed
    // in WSLENV. /p = convert path between Windows and WSL forms when needed;
    // we already pass a Linux path, so share as a plain string.
    if (useWsl) {
      const prev = env.WSLENV ?? "";
      const parts = prev
        .split(":")
        .map((s) => s.trim())
        .filter((s) => s.length > 0 && !s.startsWith("RAPTOR_LSP_BACKEND"));
      parts.push("RAPTOR_LSP_BACKEND/u");
      env.WSLENV = parts.join(":");
    }
  }

  return env;
}

function resolveBackendHostPath(): string | undefined {
  const config = workspace.getConfiguration("raptor");
  const configured = (config.get<string>("backendPath") ?? "").trim();
  if (configured.length > 0) {
    if (!path.isAbsolute(configured) && workspace.workspaceFolders?.[0]) {
      return path.join(workspace.workspaceFolders[0].uri.fsPath, configured);
    }
    return configured;
  }
  return candidateBackendBinaries().find(fileExists);
}

export function activate(context: ExtensionContext) {
  const { command, args, resolvedHostPath } = resolveServerCommand(context);
  const serverEnv = resolveBackendEnv();
  const useWsl = workspace
    .getConfiguration("raptor")
    .get<boolean>("useWsl", os.platform() === "win32");

  // Pass backend on the command line — reliable under wsl.exe (env alone is not).
  const hostBackend = resolveBackendHostPath();
  const finalArgs = [...args];
  if (hostBackend) {
    finalArgs.push("--backend", toServerPath(hostBackend, useWsl));
  }

  if (!resolvedHostPath) {
    window.showErrorMessage(
      "Raptor LSP: could not find the raptor-lsp binary. " +
        'Build it with "cargo build --release" in tools/lsp, or set "raptor.serverPath" to an absolute path.',
    );
  } else if (command !== "wsl.exe" && !fileExists(resolvedHostPath)) {
    window.showErrorMessage(
      `Raptor LSP: server binary not found at "${resolvedHostPath}". ` +
        'Build it with "cargo build --release" in tools/lsp, or set "raptor.serverPath".',
    );
  }

  if (!hostBackend) {
    window.showWarningMessage(
      "Raptor LSP: lsp-backend not found. Set raptor.backendPath to the absolute Linux path " +
        "(e.g. /mnt/c/.../build/lsp-backend) after building it with the Raptor compiler.",
    );
  }

  const serverOptions: ServerOptions = {
    run: {
      command,
      args: finalArgs,
      transport: TransportKind.stdio,
      options: { env: serverEnv },
    },
    debug: {
      command,
      args: finalArgs,
      transport: TransportKind.stdio,
      options: { env: serverEnv },
    },
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "raptor" }],
    synchronize: {
      fileEvents: workspace.createFileSystemWatcher("**/*.rp"),
    },
  };

  client = new LanguageClient(
    "raptorLanguageServer",
    "Raptor Language Server",
    serverOptions,
    clientOptions,
  );

  client.start();

  context.subscriptions.push({
    dispose: () => {
      client?.stop();
    },
  });
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
