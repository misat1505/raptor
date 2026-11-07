import * as path from "path";
import * as fs from "fs";
import * as os from "os";
import { ExtensionContext, window, workspace } from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

function windowsToWslPath(winPath: string): string {
  const match = winPath.match(/^([A-Za-z]):[\\/](.*)$/);
  if (!match) {
    return winPath.replace(/\\/g, "/");
  }
  const drive = match[1].toLowerCase();
  const rest = match[2].replace(/\\/g, "/");
  return `/mnt/${drive}/${rest}`;
}

function resolveServerCommand(context: ExtensionContext): {
  command: string;
  args: string[];
} {
  const config = workspace.getConfiguration("raptor");
  const configuredPath = config.get<string>("serverPath");
  const useWsl = config.get<boolean>("useWsl", os.platform() === "win32");

  let linuxPath: string;
  if (configuredPath && configuredPath.trim().length > 0) {
    linuxPath = configuredPath;
  } else {
    const winPath = context.asAbsolutePath(
      path.join("..", "..", "tools", "lsp", "target", "release", "raptor-lsp"),
    );
    linuxPath = useWsl ? windowsToWslPath(winPath) : winPath;
  }

  if (useWsl) {
    return { command: "wsl.exe", args: ["-e", linuxPath] };
  }
  return { command: linuxPath, args: [] };
}

export function activate(context: ExtensionContext) {
  const { command, args } = resolveServerCommand(context);

  if (command !== "wsl.exe" && !fs.existsSync(command)) {
    window.showErrorMessage(
      `Raptor LSP: server binary not found at "${command}". ` +
        `Build it with "cargo build --release" in tools/lsp (WSL), or set "raptor.serverPath".`,
    );
  }

  const serverOptions: ServerOptions = {
    run: { command, args, transport: TransportKind.stdio },
    debug: { command, args, transport: TransportKind.stdio },
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
