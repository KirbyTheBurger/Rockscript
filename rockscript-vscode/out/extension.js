"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const node_1 = require("vscode-languageclient/node");
let client;
function activate(context) {
    const serverOptions = {
        command: '/home/james/Rust/rockscript/target/debug/rockscript-lsp',
        // args: []
    };
    const clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'rockscript' }],
    };
    client = new node_1.LanguageClient('rockscriptLsp', 'RockScript Language Server', serverOptions, clientOptions);
    client.start();
    context.subscriptions.push({ dispose: () => client.stop() });
}
function deactivate() {
    return client?.stop();
}
