use axum::{Extension, Router};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod ingredients;
mod pg;
mod recipes;
mod shared;

#[derive(Clone)]
struct ApiContext {
    db: PgPool,
}

#[derive(OpenApi)]
#[openapi(
    paths(recipes::get_recipes,recipes::get_recipe, recipes::get_recipe_ingredients),
    components(schemas(recipes::recipe::Recipe, recipes::recipe_ingredient::RecipeIngredient))
)]
pub struct ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = pg::initialize_pg().await.unwrap();

    let app = Router::new()
        .nest("/ingredients", ingredients::ingredient_routes())
        .nest("/recipes", recipes::recipe_routes())
        .layer(ServiceBuilder::new().layer(Extension(ApiContext { db })))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
    // Enables logging. Use `RUST_LOG=tower_http=debug`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
