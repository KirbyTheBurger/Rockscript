import * as vscode from 'vscode';
import { LanguageClient, LanguageClientOptions, ServerOptions } from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
  const serverOptions: ServerOptions = {
    command: '/home/james/Rust/rockscript/target/debug/rockscript-lsp',
    // args: []
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: 'rockscript' }],
  };

  client = new LanguageClient(
    'rockscriptLsp',
    'RockScript Language Server',
    serverOptions,
    clientOptions
  );

  client.start();
  context.subscriptions.push({ dispose: () => client.stop() });
}

export function deactivate(): Thenable<void> | undefined {
  return client?.stop();
}