use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct RecipeIngredient {
    pub ingredient_id: i64,
    pub name: String,
    #[sqlx(rename = "type")]
    pub kind: String,
    pub quantity: f32,
}
