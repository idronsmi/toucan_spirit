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
            Some(limit) => limit,
            None => 10,
        };

        let offset = match self.offset {
            Some(offset) => offset,
            None => 0,
        };

        format!("LIMIT {limit} OFFSET {offset}")
    }
}
