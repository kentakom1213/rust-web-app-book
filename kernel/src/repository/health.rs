use async_trait::async_trait;

/// データベースのヘルスチェックを行う
#[async_trait]
pub trait HealthCheckRepository: Send + Sync {
    /// データベースに接続し，接続を確立できるか判定する
    async fn check_db(&self) -> bool;
}
