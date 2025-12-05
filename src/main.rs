#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = quill::Server::new();

    app.run().await
}
