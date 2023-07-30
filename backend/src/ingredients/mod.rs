use axum::{extract::Query, http::StatusCode, routing::get, Extension, Json, Router};

use ingredient::Ingredient;

use crate::{shared, ApiContext};

pub mod ingredient;

pub fn ingredient_routes() -> Router {
    let ingredients_router = Router::new().route("/", get(get_ingredients));
    ingredients_router
}

async fn get_ingredients(
    ctx: Extension<ApiContext>,
    pagination: Option<Query<shared::pagination::Pagination>>,
) -> Result<Json<Vec<Ingredient>>, (StatusCode, String)> {
    let Query(pagination) = pagination.unwrap_or_default();
    let paginated = pagination.make_query_string();

    let query = format!("SELECT ingredient_id, name, type FROM tbl_ingredient {paginated}");

    let sql = sqlx::query_as::<_, Ingredient>(&query);
    let ings = sql.fetch_all(&ctx.db).await.unwrap();

    Ok(Json(ings))
}
