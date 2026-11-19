use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // Jalur endpoint utama (/)
    let app = Router::new().route("/", get(|| async { "Halo dari Rust di Termux via Cloudflare!" }));

    // Bind ke port 8080 sesuai setup tunnel Anda
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server Rust berjalan di http://127.0.0.1:8080");
    
    axum::serve(listener, app).await.unwrap();
}


