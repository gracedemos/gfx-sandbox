mod app;
mod vertex;
mod pipeline;
mod texture;
mod mesh;

#[tokio::main]
async fn main() {
    let app_result = app::run().await;

    if let Err(e) = app_result {
        eprintln!("Error: {e}");
    }
}
