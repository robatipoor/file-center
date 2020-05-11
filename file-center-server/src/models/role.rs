use sqlx::decode::Decode;
use sqlx::encode::Encode;
use sqlx::prelude::*;
use sqlx::sqlite::SqliteTypeInfo;
use sqlx::{FromRow, Sqlite, Type};
use sqlx::{Pool, SqliteConnection};
use std::string::ToString;

#[derive(FromRow, Debug)]
pub struct Role {
    pub id: i64,
    pub role_name: RoleName,
}

#[derive(Decode, Encode, Debug, Copy, Clone, Display)]
pub enum RoleName {
    ADMIN = 1,
    USER,
}

impl From<String> for RoleName {
    fn from(input: String) -> RoleName {
        use RoleName::*;
        let input_uppercase = input.to_uppercase();
        if input_uppercase == ADMIN.to_string() {
            ADMIN
        } else if input_uppercase == USER.to_string() {
            USER
        } else {
            panic!("input invalid !");
        }
    }
}

impl Type<Sqlite> for RoleName {
    fn type_info() -> SqliteTypeInfo {
        <str as Type<Sqlite>>::type_info()
    }
}

impl Role {
    pub async fn new(role_name: RoleName) -> anyhow::Result<Role> {
        Ok(Role {
            id: role_name as i64,
            role_name,
        })
    }

    pub async fn find_by_name(
        pool: &Pool<SqliteConnection>,
        role_name: RoleName,
    ) -> anyhow::Result<Role> {
        Ok(
            sqlx::query_as::<_, Role>("SELECT id,role_name FROM roles WHERE role_name = $1")
                .bind(role_name.to_string())
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn find_by_id(pool: &Pool<SqliteConnection>, user_id: i64) -> anyhow::Result<Role> {
        Ok(
            sqlx::query_as::<_, Role>("SELECT id,role_name FROM roles WHERE id = $1")
                .bind(user_id)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn find_all(pool: &Pool<SqliteConnection>) -> anyhow::Result<Vec<Role>> {
        Ok(sqlx::query_as::<_, Role>("SELECT id,role_name FROM roles")
            .fetch_all(pool)
            .await?)
    }
}
