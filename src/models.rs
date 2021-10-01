use crate::types::DbPool;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Row;

#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Askee {
    #[serde(default = "default_id")]
    pub id: i32,
    pub display_name: String,
    #[serde(default, with = "timestamp_option")]
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ask {
    #[serde(default = "default_id")]
    pub id: i32,
    pub askee: i32,
    pub content: String,
    #[serde(default, with = "timestamp_option")]
    pub created_at: Option<NaiveDateTime>,
    pub dedup: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeRange {
    #[serde(default, with = "timestamp_option")]
    pub before: Option<NaiveDateTime>,
    #[serde(default, with = "timestamp_option")]
    pub after: Option<NaiveDateTime>,
}

impl Askee {
    /// create a new Askee with a display name
    pub async fn create<D: AsRef<str>>(pool: &DbPool, display_name: D) -> Result<i32, sqlx::Error> {
        let id: i32 = sqlx::query("insert into askee (display_name) values ($1) returning id")
            .bind(display_name.as_ref())
            .fetch_one(pool)
            .await?
            .get(0);
        Ok(id)
    }

    /// load a single Askee
    pub async fn load(pool: &DbPool, id: i32) -> Result<Self, sqlx::Error> {
        let askee: Self = sqlx::query_as("select * from askee where id = $1 limit 1")
            .bind(id)
            .fetch_one(pool)
            .await?;
        Ok(askee)
    }

    /// WARNING: no limit on how many rows should return. (Potentially cause a DoS attack)
    /// TODO: add paging
    pub async fn fetch_all(pool: &DbPool) -> Result<Vec<Self>, sqlx::Error> {
        let askees: Vec<Self> = sqlx::query_as("select * from askee order by id")
            .fetch_all(pool)
            .await?;
        Ok(askees)
    }
}

impl Ask {
    pub async fn create<C: AsRef<str>, D: AsRef<str>>(
        pool: &DbPool,
        askee: i32,
        content: C,
        dedup: D,
    ) -> Result<i32, sqlx::Error> {
        let id: i32 =
            sqlx::query("insert into ask (askee, content, dedup) values ($1, $2, $3) returning id")
                .bind(askee)
                .bind(content.as_ref())
                .bind(dedup.as_ref())
                .fetch_one(pool)
                .await?
                .get(0);
        Ok(id)
    }

    pub async fn load(pool: &DbPool, id: i32) -> Result<Self, sqlx::Error> {
        let ask: Self = sqlx::query_as("select * from ask where id = $1 limit 1")
            .bind(id)
            .fetch_one(pool)
            .await?;
        Ok(ask)
    }

    /// fetch all ask to a user in a time range, must give a time range
    pub async fn fetch_all(
        pool: &DbPool,
        id: i32,
        before: NaiveDateTime,
        after: NaiveDateTime,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let asks: Vec<Self> = sqlx::query_as("select * from ask where askee = $1 where created_at < $2 and created_at > $3 order by created_at")
            .bind(id)
            .bind(before)
            .bind(after)
            .fetch_all(pool)
            .await?;
        Ok(asks)
    }
}

const fn default_id() -> i32 {
    -1
}

mod timestamp_option {
    use chrono::NaiveDateTime;
    use serde::de::Error;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S.%f";

    pub fn serialize<S>(dt: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match dt {
            None => serializer.serialize_none(),
            Some(dt) => serializer.serialize_some(&dt.format(FORMAT).to_string()),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let formatted = Option::<String>::deserialize(deserializer)?;
        let result = match formatted {
            None => None,
            Some(formatted) => {
                Some(NaiveDateTime::parse_from_str(&*formatted, FORMAT).map_err(D::Error::custom)?)
            }
        };
        Ok(result)
    }
}
