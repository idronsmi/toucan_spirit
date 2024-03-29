use sqlx::sqlx_macros::migrate;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn initialize_pg() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://toucan-spirit-sa:dontusemeinproduction@192.168.0.39/toucan_spirit")
        .await?;

    migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
