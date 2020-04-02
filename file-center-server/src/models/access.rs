use sqlx::decode::Decode;
use sqlx::encode::Encode;
use sqlx::prelude::*;
use sqlx::sqlite::SqliteTypeInfo;
use sqlx::Row;
use sqlx::{Pool, SqliteConnection};
use sqlx::{Sqlite, Type};
use std::string::ToString;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug)]
pub struct Access {
    pub id: i32,
    pub access_type: AccessType,
}

#[derive(Debug, Decode, Encode, Copy, Clone, Display,Serialize, Deserialize)]
pub enum AccessType {
    Read,
    Write,
}

impl Type<Sqlite> for AccessType {
    fn type_info() -> SqliteTypeInfo {
        <str as Type<Sqlite>>::type_info()
    }
}

impl From<String> for AccessType {
    fn from(input: String) -> AccessType {
        use AccessType::*;
        let input_uppercase = input.to_uppercase();
        if input_uppercase == Read.to_string() {
            return Read;
        } else if input_uppercase == Write.to_string() {
            return Write;
        } else {
            panic!("input invalid !");
        }
    }
}

impl Access {
    pub async fn new(access_type: AccessType) -> anyhow::Result<Access> {
        Ok(Access {
            id: access_type as i32,
            access_type,
        })
    }

    pub async fn find_by_name(
        pool: &Pool<SqliteConnection>,
        access_type: AccessType,
    ) -> anyhow::Result<Access> {
        let access = sqlx::query_as::<_, Access>(
            "SELECT id, access_type FROM access WHERE access_type = $1",
        )
        .bind(access_type.to_string())
        .fetch_one(pool)
        .await?;
        Ok(access)
    }

    pub async fn find_by_id(
        pool: &Pool<SqliteConnection>,
        access_id: i64,
    ) -> anyhow::Result<Access> {
        let access =
            sqlx::query_as::<_, Access>("SELECT id, access_type FROM access WHERE id = $1")
                .bind(access_id)
                .fetch_one(pool)
                .await?;
        Ok(access)
    }

    pub async fn find_all(pool: &Pool<SqliteConnection>) -> anyhow::Result<Vec<Access>> {
        let access = sqlx::query_as::<_, Access>("SELECT id, access_type FROM access ")
            .fetch_all(pool)
            .await?;
        Ok(access)
    }
}
