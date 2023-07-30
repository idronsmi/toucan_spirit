use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct RecipeIngredient {
    pub ingredient_id: i64,
    pub name: String,
    #[sqlx(rename = "type")]
    pub kind: String,
    pub quantity: f32,
}
