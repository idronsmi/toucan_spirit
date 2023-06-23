use axum::{
    extract::{Path, Query},
    http::StatusCode,
    routing::get,
    Extension, Json, Router,
};

use crate::{shared::pagination::Pagination, ApiContext};

use recipe::Recipe;
use recipe_ingredient::RecipeIngredient;

pub mod recipe;
pub mod recipe_ingredient;

pub fn recipe_routes() -> Router {
    let router = Router::new()
        .route("/", get(get_recipes))
        .route("/:recipe_id/ingredients", get(get_recipe_ingredients));
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

async fn get_recipe_ingredients(
    Path(recipe_id): Path<i64>,
    ctx: Extension<ApiContext>,
) -> Result<Json<Vec<RecipeIngredient>>, (StatusCode, String)> {
    let query = format!(
        "
       SELECT I.ingredient_id, I.name, type, quantity 
       FROM tbl_recipe_ingredient as RI
       JOIN tbl_ingredient as I
            ON I.ingredient_id = RI.ingredient_id
        WHERE RI.recipe_id = $1
        "
    );

    let sql = sqlx::query_as::<_, RecipeIngredient>(&query).bind(recipe_id);
    let ings = sql.fetch_all(&ctx.db).await.unwrap();

    Ok(Json(ings))
}
