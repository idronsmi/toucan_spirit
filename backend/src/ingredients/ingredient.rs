use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct Ingredient {
    pub ingredient_id: i64,
    pub name: String,
    #[sqlx(rename = "type")]
    pub kind: String,
}
