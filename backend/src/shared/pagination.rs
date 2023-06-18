use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    offset: Option<usize>,
    limit: Option<usize>,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            offset: Some(0),
            limit: Some(10),
        }
    }
}

impl Pagination {
    pub fn make_query_string(&self) -> String {
        let limit = match self.limit {
            Some(limit) => format!("LIMIT {limit}"),
            None => "LIMIT 10".to_owned(),
        };

        let offset = match self.offset {
            Some(offset) => format!("OFFSET {offset}"),
            None => "OFFSET 0".to_owned(),
        };

        format!("{limit} {offset}")
    }
}
