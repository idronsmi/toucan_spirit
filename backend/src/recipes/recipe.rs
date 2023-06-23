use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Recipe {
    pub recipe_id: i64,
    pub name: String,
    pub how: String,
}
