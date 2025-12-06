use quill::Server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Server::default();

    app.run().await
}
