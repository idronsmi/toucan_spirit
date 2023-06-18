use axum::{extract::Query, http::StatusCode, routing::get, Extension, Json, Router};

use ingredient::Ingredient;

use crate::{shared::pagination::Pagination, ApiContext};

mod ingredient;

pub fn get_ingredients_routes() -> Router {
    let ingredients_router = Router::new().route("/", get(get_ingredients));
    ingredients_router
}

async fn get_ingredients(
    ctx: Extension<ApiContext>,
    pagination: Option<Query<Pagination>>,
) -> Result<Json<Vec<Ingredient>>, (StatusCode, String)> {
    let Query(pagination) = pagination.unwrap_or_default();
    let query = String::from("SELECT ingredient_id, name, type FROM tbl_ingredient ");
    let paginated = pagination.make_query_string();
    let paginated_query = query + &paginated;

    let sql = sqlx::query_as::<_, Ingredient>(&paginated_query);
    let ings = sql.fetch_all(&ctx.db).await.unwrap();

    Ok(Json(ings))
}
