use std::collections::HashMap;

use rockscript_core::{error::Diagnostic as RockscriptDiagnostic, lexer::{Token, tokenize}, parser::Parser};
use tokio::sync::RwLock;
use tower_lsp_server::{Client, LanguageServer, ls_types::{Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, Hover, HoverContents, HoverParams, HoverProviderCapability, InitializeParams, InitializeResult, InitializedParams, MarkedString, MessageType, Position, Range, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, Uri}};

#[derive(Debug)]
pub struct Backend {
    client: Client,
    documents: RwLock<HashMap<Uri, String>>,
}

impl Backend {
    pub fn new(client: Client) -> Backend {
        Backend {
            client,
            documents: RwLock::new(HashMap::new()),
        }
    }

    async fn insert_doc(&self, uri: Uri, text: String) {
        self.documents.write().await.insert(uri, text);
    }

    async fn check_document(&self, uri: Uri, text: &str) {
        let mut diagnostics = vec![];

        match tokenize(text, false) {
            Ok(tokens) => {
                if let Err(errs) = Parser::new(tokens, false).parse() {
                    for e in errs {
                        diagnostics.push(to_lsp_diagnostic(text, &e));
                    }
                }
            },
            Err(errs) => {
                for e in errs {
                    diagnostics.push(to_lsp_diagnostic(text, &e));
                }
            }
        }

        self.client.publish_diagnostics(uri, diagnostics, None).await
    }
}

impl LanguageServer for Backend {
    async fn initialize(
        &self,
        _: InitializeParams
    ) -> tower_lsp_server::jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client.log_message(MessageType::INFO, "server initialized").await;
    }

    async fn shutdown(&self) -> tower_lsp_server::jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let text = params.text_document.text;
        let uri = params.text_document.uri;

        self.insert_doc(uri.clone(), text.clone()).await;
        self.check_document(
            uri,
            &text,
        ).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().next() {
            let uri = params.text_document.uri;
            let text = change.text;

            self.insert_doc(uri.clone(), text.clone()).await;
            self.check_document(
                uri,
                &text
            ).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.client.publish_diagnostics(
            params.text_document.uri,
            vec![],
            None,
        ).await;
    }

    async fn hover(&self, params: HoverParams) -> tower_lsp_server::jsonrpc::Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let text = match self.documents.read().await.get(&uri) {
            Some(t) => t.clone(),
            None => return Ok(None),
        };
        let offset = position_to_offset(&text, pos);

        let tokens = match tokenize(&text, false) {
            Ok(t) => t,
            Err(_) => return Ok(None),
        };

        let hovered = match tokens.iter()
            .find(|t| t.span.contains(&offset)) {
                Some(t) => t,
                None => return Ok(None),
            };
        
        let message = match &hovered.token {
            Token::String(s) => format!("String literal: `{s}`"),
            Token::Number(n) => format!("Number literal: `{n}`"),
            Token::True => format!("Boolean literal: `big`"),
            Token::False => format!("Boolean literal: `small`"),
            Token::Identifier(s) => format!("Identifier: `{s}`"),

            Token::Throw | Token::Rock | Token::At => format!("Keyword inside variable definition"),
            Token::Named => format!("Keyword inside string variable definition"),

            Token::Present => format!("Built in print function"),

            Token::Smash => format!("Addition operator"),
            Token::Chip => format!("Subtraction operator"),
            Token::Mate => format!("Multiplication operator"),
            Token::Split => format!("Division operator"),
            Token::Into => format!("Keyword for defining functions / Addition operator helper"),
            Token::Off => format!("Subtraction operator helper"),
            Token::With => format!("Keyword for passing an argument into a function / Multiplication operator helper"),
            Token::From => format!("Division operator helper"),

            Token::LParen | Token::RParen => return Ok(None),

            Token::Carve | Token::Instruction => format!("Keywoord for defining functions"),
            Token::Retrieve => format!("Keyword for accessing function arguments"),
            Token::Follow => format!("Keyword for calling functions"),
            Token::And => format!("Keyword for passing more than 1 argument into a function"),
            Token::Engrave => format!("Keyword for returning values"),

            Token::Enough => format!("Keyword to indicate the end of a code block"),
            
            Token::Weigh => format!("Comparison operator (x >= y)"),
            Token::Against => format!("Comparison operator helper"),

            Token::Inspect => format!("Keyword to start an if statement"),
            Token::Refine => format!("Keyword to start an else statement"),

            Token::Roll | Token::While => format!("Keyword inside while loop"),
            Token::Destroy => format!("Keyword to break a while loop"),
        };

        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(message)),
            range: Some(span_to_range(&text, hovered.span.clone())),
        }))
    }
}

fn position_to_offset(source: &str, pos: Position) -> usize {
    let mut current_line = 0u32;
    let mut offset = 0usize;

    for (i, c) in source.char_indices() {
        if current_line == pos.line {
            let line_start = i;
            let mut char_count = 0u32;
            for (j, _) in source[line_start..].char_indices() {
                if char_count == pos.character {
                    return line_start + j;
                }
                char_count += 1;
            }
        }
        if c == '\n' {
            current_line += 1;
        }
        offset = i;
    }
    offset
}


fn offset_to_position(source: &str, byte_offset: usize) -> Position {
    let mut line = 0u32;
    let mut character = 0u32;

    for (i, c) in source.char_indices() {
        if i >= byte_offset {
            break;
        }
        if c == '\n' {
            line += 1;
            character = 0;
        } else {
            character += 1;
        }
    }

    Position { line, character }
}

fn span_to_range(source: &str, span: std::ops::Range<usize>) -> Range {
    Range {
        start: offset_to_position(source, span.start),
        end: offset_to_position(source, span.end),
    }
}

fn to_lsp_diagnostic(source: &str, err: &dyn RockscriptDiagnostic) -> Diagnostic {
    Diagnostic {
        range: span_to_range(source, err.span()),
        severity: Some(DiagnosticSeverity::ERROR),
        message: err.desc(),
        ..Default::default()
    }
}
