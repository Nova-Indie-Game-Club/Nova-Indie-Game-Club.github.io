use notion::models::search::DatabaseQuery;

pub trait Empty {
    fn empty() -> Self;
}

impl Empty for DatabaseQuery {
    fn empty() -> Self {
        DatabaseQuery {
            sorts: None,
            filter: None,
            paging: None,
        }
    }
}
