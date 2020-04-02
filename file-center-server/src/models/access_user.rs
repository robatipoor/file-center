use sqlx::prelude::*;
use sqlx::{Pool, SqliteConnection};
#[derive(sqlx::FromRow, Debug)]
pub struct AccessUser {
    pub id: i32,
    pub user_id: i32,
    pub file_id: i32,
    pub access_id: i32,
}

impl AccessUser {
    pub async fn new(user_id: i32, file_id: i32, access_id: i32) -> anyhow::Result<AccessUser> {
        Ok(AccessUser {
            id: 0,
            user_id,
            file_id,
            access_id,
        })
    }

    pub async fn save(&self, pool: &Pool<SqliteConnection>) -> anyhow::Result<i64> {
        sqlx::query!(
            r#"INSERT INTO access_users (user_id , file_id , access_id) VALUES ($1,$2,$3)"#,
            self.user_id,
            self.file_id,
            self.access_id,
        )
        .execute(pool)
        .await?;
        let record: (i64,) = sqlx::query_as("SELECT last_insert_rowid()")
            .fetch_one(pool)
            .await?;
        Ok(record.0)
    }

    pub async fn find_by_id(
        pool: &Pool<SqliteConnection>,
        access_user_id: i64,
    ) -> anyhow::Result<AccessUser> {
        let access = sqlx::query_as::<_, AccessUser>(
            "SELECT id, user_id ,file_id , access_id FROM access_users WHERE id = $1",
        )
        .bind(access_user_id)
        .fetch_one(pool)
        .await?;
        Ok(access)
    }

    pub async fn find_by_user_id(
        pool: &Pool<SqliteConnection>,
        user_id: i64,
    ) -> anyhow::Result<Vec<AccessUser>> {
        let access = sqlx::query_as::<_, AccessUser>(
            "SELECT id, user_id ,file_id , access_id FROM access_users WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;
        Ok(access)
    }

    pub async fn find_by_file_id(
        pool: &Pool<SqliteConnection>,
        file_id: i64,
    ) -> anyhow::Result<Vec<AccessUser>> {
        let access = sqlx::query_as::<_, AccessUser>(
            "SELECT id, user_id ,file_id , access_id FROM access_users WHERE file_id = $1",
        )
        .bind(file_id)
        .fetch_all(pool)
        .await?;
        Ok(access)
    }

    pub async fn find_id(
        pool: &Pool<SqliteConnection>,
        user_id: i64,
        file_id: i64,
    ) -> anyhow::Result<i64> {
        let access: (i64,) =
            sqlx::query_as("SELECT id FROM access_users WHERE user_id = $1 AND file_id = $2")
                .bind(user_id)
                .bind(file_id)
                .fetch_one(pool)
                .await?;
        Ok(access.0)
    }

    pub async fn is_user_access(
        pool: &Pool<SqliteConnection>,
        user_id: i64,
        file_id: i64,
        access_id: i64,
    ) -> anyhow::Result<bool> {
        let access:(i64,) = sqlx::query_as(
            "SELECT EXISTS (SELECT id FROM access_users WHERE user_id = $1 AND file_id = $2 AND access_id = $3)",
        )
        .bind(user_id)
        .bind(file_id)
        .bind(access_id)
        .fetch_one(pool)
        .await?;
        Ok(access.0 > 0)
    }

    pub async fn user_has_access_by_link(
        pool: &Pool<SqliteConnection>,
        link: &str,
        user_id: i64,
    ) -> anyhow::Result<bool> {
        let access: (i64,) = sqlx::query_as(
            "SELECT EXISTS (SELECT id FROM access_users WHERE user_id = $1 AND file_id IN (SELECT id FROM files WHERE link = $2))",
        )
        .bind(user_id)
        .bind(link)
        .fetch_one(pool)
        .await?;
        Ok(access.0 > 0)
    }

    pub async fn user_has_access(
        pool: &Pool<SqliteConnection>,
        file_id: i64,
        user_id: i64,
    ) -> anyhow::Result<bool> {
        let access: (i64,) = sqlx::query_as(
            "SELECT EXISTS (SELECT id FROM access_users WHERE user_id = $1 AND file_id = $2)",
        )
        .bind(user_id)
        .bind(file_id)
        .fetch_one(pool)
        .await?;
        Ok(access.0 > 0)
    }

    pub async fn update_access(
        pool: &Pool<SqliteConnection>,
        access_user_id: i64,
        access_id: i64,
    ) -> anyhow::Result<u64> {
        let row_affected = sqlx::query!(
            r#"UPDATE access_users SET access_id = ?1 WHERE id = ?2"#,
            access_id,
            access_user_id
        )
        .execute(pool)
        .await?;
        Ok(row_affected)
    }

    pub async fn update(&self, pool: &Pool<SqliteConnection>) -> anyhow::Result<u64> {
        let row_affected = sqlx::query!(
            r#"UPDATE access_users SET user_id = $1 ,file_id = $2 ,access_id = $3 WHERE id = $4"#,
            self.user_id,
            self.file_id,
            self.access_id,
            self.id
        )
        .execute(pool)
        .await?;
        Ok(row_affected)
    }

    pub async fn delete(&self, pool: &Pool<SqliteConnection>) -> anyhow::Result<u64> {
        let row_affected = sqlx::query!(r#"DELETE FROM access_users WHERE id = $1"#, self.id)
            .execute(pool)
            .await?;
        Ok(row_affected)
    }
}
