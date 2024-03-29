use crate::config::CFG;
use sqlx::SqlitePool;
use tokio::sync::OnceCell;
pub static DB: OnceCell<SqlitePool> = OnceCell::const_new();
pub async fn init_db_conn() {
    DB.get_or_init(|| async {
        SqlitePool::connect(&CFG.database.database_url)
            .await
            .expect("数据库连接失败。")
    })
    .await;
}
