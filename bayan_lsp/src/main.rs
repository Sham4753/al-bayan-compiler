use tower_lsp::{LspService, Server};
use tokio::io::{stdin, stdout};

mod server;

#[tokio::main]
async fn main() {
    let (service, socket) = LspService::new(|client| server::BayanLanguageServer::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
