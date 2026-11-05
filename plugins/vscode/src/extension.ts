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

// Konwertuje ścieżkę Windows (C:\Users\...) na ścieżkę WSL (/mnt/c/Users/...)
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
  // Nadpisanie ręczne w settings.json:
  // "raptor.serverPath": "/mnt/c/.../target/release/lsp"  (ścieżka WSL)
  // "raptor.useWsl": true/false
  const config = workspace.getConfiguration("raptor");
  const configuredPath = config.get<string>("serverPath");
  const useWsl = config.get<boolean>("useWsl", os.platform() === "win32");

  let linuxPath: string;
  if (configuredPath && configuredPath.trim().length > 0) {
    linuxPath = configuredPath;
  } else {
    // Domyślnie: repo obok vscode-ext/, binarka w target/release/lsp
    const winPath = context.asAbsolutePath(
      path.join("..", "target", "release", "lsp"),
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

  // Szybka weryfikacja tylko dla trybu bez WSL (lokalny plik da się sprawdzić fs.existsSync)
  if (command !== "wsl.exe" && !fs.existsSync(command)) {
    window.showErrorMessage(
      `Raptor LSP: nie znaleziono binarki serwera pod "${command}". ` +
        `Zbuduj ją poleceniem "cargo build --release --bin lsp" w WSL, albo ustaw "raptor.serverPath".`,
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
