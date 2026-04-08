use diom_error::Result;
use tokio::sync::OnceCell;

const BENCH_PG_URL: &str = "postgres://postgres:postgres@localhost/benchmark";

static PG_POOL: OnceCell<sqlx::PgPool> = OnceCell::const_new();

async fn pool() -> &'static sqlx::PgPool {
    PG_POOL
        .get_or_init(|| async {
            let pool = sqlx::PgPool::connect(BENCH_PG_URL)
                .await
                .expect("benchmark: failed to connect to postgres");
            sqlx::query(
                "CREATE TABLE IF NOT EXISTS diom_bench_kv \
                 (key TEXT PRIMARY KEY, value BYTEA NOT NULL)",
            )
            .execute(&pool)
            .await
            .expect("benchmark: failed to create diom_bench_kv table");
            pool
        })
        .await
}

pub async fn pg_set(key: &str, value: &[u8]) -> Result<()> {
    sqlx::query(
        "INSERT INTO diom_bench_kv (key, value) VALUES ($1, $2) \
         ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value",
    )
    .bind(key)
    .bind(value)
    .execute(pool().await)
    .await
    .map_err(|e| diom_error::Error::internal(e))?;
    Ok(())
}

pub async fn pg_fetch(key: &str) -> Result<Option<Vec<u8>>> {
    use sqlx::Row as _;
    let row = sqlx::query("SELECT value FROM diom_bench_kv WHERE key = $1")
        .bind(key)
        .fetch_optional(pool().await)
        .await
        .map_err(|e| diom_error::Error::internal(e))?;
    Ok(row.map(|r| r.get::<Vec<u8>, _>(0)))
}
