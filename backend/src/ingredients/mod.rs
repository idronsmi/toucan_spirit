use axum::{Router, Extension, debug_handler,response::IntoResponse,
    routing::{get}, Json};
use sqlx::{PgPool};

use ingredient::Ingredient;

mod ingredient;

pub fn get_ingredients_routes() -> Router {
    let ingredients_router = Router::new().route("/", get(get_ingredients));
    ingredients_router
}

#[debug_handler]
async fn get_ingredients(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let ings = sqlx::query_as::<_, Ingredient>("SELECT ingredient_id, name, type FROM tbl_ingredient").fetch_all(&pool).await.unwrap();
    Json(ings)
}