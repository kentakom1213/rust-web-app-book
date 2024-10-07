use std::net::{Ipv4Addr, SocketAddr};

use adapter::database::connect_database_with;
use anyhow::{Error, Result};
use api::route::health::build_health_check_routers;
use axum::Router;
use registry::AppRegistry;
use shared::config::AppConfig;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    bootstrap().await
}

async fn bootstrap() -> Result<()> {
    // app_configの生成
    let app_config = AppConfig::new()?;
    // データベースへの接続
    let pool = connect_database_with(&app_config.database);

    // AppRegistryを生成する
    let registry = AppRegistry::new(pool);

    // build_health_check関数を呼び出す
    let app = Router::new()
        // ルーターに登録する
        .merge(build_health_check_routers())
        .with_state(registry);

    // サーバーの起動
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on {}", addr);

    axum::serve(listener, app).await.map_err(Error::from)
}
