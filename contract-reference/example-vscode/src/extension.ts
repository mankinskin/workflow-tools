import * as vscode from 'vscode';

// Reference default: matches `example-viewer`'s `ServerConfig::new("example-viewer", 3099)`.
const DEFAULT_VIEWER_URL = 'http://127.0.0.1:3099';

export function activate(context: vscode.ExtensionContext): void {
  const openBrowser = vscode.commands.registerCommand(
    'example-viewer.openBrowser',
    async () => {
      await vscode.env.openExternal(vscode.Uri.parse(DEFAULT_VIEWER_URL));
    },
  );

  context.subscriptions.push(openBrowser);
}

export function deactivate(): void {}
