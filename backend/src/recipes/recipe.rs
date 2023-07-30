use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct Recipe {
    pub recipe_id: i64,
    pub name: String,
    pub how: String,
}
