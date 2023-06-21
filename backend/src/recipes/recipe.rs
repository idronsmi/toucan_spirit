use serde::{Deserialize, Serialize};

use crate::ingredients::ingredient::Ingredient;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Recipe {
    pub recipe_id: i64,
    pub name: String,
    pub how: String,
    pub ingredients: Vec<Ingredient>,
}
