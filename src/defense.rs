use crate::models::Askee;
use crate::types::DbPool;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct AskeeFilter {
    filter: HashMap<i32, Askee>,
}

impl AskeeFilter {
    pub async fn new(pool: &DbPool) -> Result<Self, sqlx::Error> {
        let askees: Vec<Askee> = sqlx::query_as("select * from askee")
            .fetch_all(pool)
            .await?;
        let askees: HashMap<i32, Askee> =
            askees.into_iter().map(|askee| (askee.id, askee)).collect();
        Ok(Self { filter: askees })
    }
}
