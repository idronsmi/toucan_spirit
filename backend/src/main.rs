use axum::{
 Router, Extension
};
use std::net::SocketAddr;
use tower::ServiceBuilder;

mod pg;
mod ingredients;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = pg::initialize_pg().await.unwrap();

    let app = Router::new()

        .nest("/ingredients", ingredients::get_ingredients_routes()).layer(
            ServiceBuilder::new()
                .layer(Extension(pool))
        );


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
