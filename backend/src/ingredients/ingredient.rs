use serde::Deserialize;




#[derive(Debug, sqlx::FromRow, Deserialize)]
pub struct Ingredient {
    pub ingredient_id: i64,
    pub name: String,
    #[sqlx(rename = "type")]
    pub kind: String,
}