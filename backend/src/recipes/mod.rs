use axum::{extract::Query, http::StatusCode, routing::get, Extension, Json, Router};

use crate::{shared::pagination::Pagination, ApiContext};

use recipe::Recipe;

pub mod recipe;

pub fn recipe_routes() -> Router {
    let router = Router::new().route("/", get(get_recipes));
    router
}

async fn get_recipes(
    ctx: Extension<ApiContext>,
    pagination: Option<Query<Pagination>>,
) -> Result<Json<Vec<Recipe>>, (StatusCode, String)> {
    let Query(pagination) = pagination.unwrap_or_default();
    let paginated = pagination.make_query_string();

    let query = format!(
        "
        SELECT recipe_id, name, how 
        FROM tbl_recipe as R
        JOIN tbl_prep as P
            ON P.prep_id = R.prep_id
        {paginated}
        "
    );

    let sql = sqlx::query_as::<_, Recipe>(&query);
    let ings = sql.fetch_all(&ctx.db).await.unwrap();

    Ok(Json(ings))
}
