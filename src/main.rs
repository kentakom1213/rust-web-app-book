use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{extract::State, http::StatusCode, routing::get, Router};
use sqlx::{postgres::PgConnectOptions, PgPool};
use tokio::net::TcpListener;

/// データベースの接続設定
struct DatabaseConig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl From<DatabaseConig> for PgConnectOptions {
    fn from(cfg: DatabaseConig) -> Self {
        Self::new()
            .host(&cfg.host)
            .port(cfg.port)
            .username(&cfg.username)
            .password(&cfg.password)
            .database(&cfg.database)
    }
}

/// postgresl用のコネクションプール
fn connect_database_with(cfg: DatabaseConig) -> PgPool {
    PgPool::connect_lazy_with(cfg.into())
}

/// ヘルスチェック用のハンドラ
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

/// DBのヘルスチェックを行うハンドラ
async fn health_check_db(State(db): State<PgPool>) -> StatusCode {
    let connection_result = sqlx::query("SELECT 1").fetch_one(&db).await;
    match connection_result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tokio::test]
async fn health_check_works() {
    let status_code = health_check().await;
    assert_eq!(status_code, StatusCode::OK);
}

#[tokio::main]
async fn main() -> Result<()> {
    // DBの設定
    let database_cfg = DatabaseConig {
        host: "localhost".into(),
        port: 5432,
        username: "app".into(),
        password: "passwd".into(),
        database: "app".into(),
    };

    // コネクションプールを作る
    let conn_pool = connect_database_with(database_cfg);

    // ルーター
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/health/db", get(health_check_db))
        .with_state(conn_pool);

    // ローカルホストの8080番
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    // リスナーを立ち上げる
    let listener = TcpListener::bind(addr).await?;

    // ログを出力する
    println!("Listening on {}", addr);

    // サーバーを起動する
    Ok(axum::serve(listener, app).await?)
}
