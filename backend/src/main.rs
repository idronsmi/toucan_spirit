use axum::{Extension, Router};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower::ServiceBuilder;

mod ingredients;
mod pg;
mod recipes;
mod shared;

#[derive(Clone)]
struct ApiContext {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = pg::initialize_pg().await.unwrap();

    let app = Router::new()
        .nest("/ingredients", ingredients::ingredient_routes())
        .nest("/recipes", recipes::recipe_routes())
        .layer(ServiceBuilder::new().layer(Extension(ApiContext { db })));
    // Enables logging. Use `RUST_LOG=tower_http=debug`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
