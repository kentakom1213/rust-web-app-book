use std::net::{Ipv4Addr, SocketAddr};

use axum::{routing::get, Router};
use tokio::net::TcpListener;

async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    // ルーター
    let app = Router::new().route("/hello", get(hello_world));

    // ローカルホストの8080番
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    // リスナーを立ち上げる
    let listener = TcpListener::bind(addr).await.unwrap();

    // ログを出力する
    println!("Listening on {}", addr);

    // サーバーを起動する
    axum::serve(listener, app).await.unwrap();
}
