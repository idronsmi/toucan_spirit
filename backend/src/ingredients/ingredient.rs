use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Ingredient {
    pub ingredient_id: i64,
    pub name: String,
    #[sqlx(rename = "type")]
    pub kind: String,
}
