use sqlx::sqlx_macros::migrate;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async  fn initialize_pg() -> Result<PgPool, sqlx::Error> {

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://postgres:admin@localhost/toucan_spirit").await?;

    migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}
