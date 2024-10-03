use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;

/// ヘルスチェック用のハンドラ
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() -> Result<()> {
    // ルーター
    let app = Router::new().route("/health", get(health_check));

    // ローカルホストの8080番
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    // リスナーを立ち上げる
    let listener = TcpListener::bind(addr).await?;

    // ログを出力する
    println!("Listening on {}", addr);

    // サーバーを起動する
    Ok(axum::serve(listener, app).await?)
}
