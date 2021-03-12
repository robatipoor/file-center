use sqlx::SqlitePool;
#[derive(sqlx::FromRow, Debug)]
pub struct AccessUser {
    pub id: i64,
    pub user_id: i64,
    pub file_id: i64,
    pub access_id: i64,
}

impl AccessUser {
    pub async fn new(user_id: i64, file_id: i64, access_id: i64) -> anyhow::Result<AccessUser> {
        Ok(AccessUser {
            id: 0,
            user_id,
            file_id,
            access_id,
        })
    }

    pub async fn save(&self, pool: &SqlitePool) -> anyhow::Result<i64> {
        sqlx::query(
            r#"INSERT INTO access_users (user_id , file_id , access_id) VALUES ($1,$2,$3)"#,
        )
        .bind(self.user_id)
        .bind(self.file_id)
        .bind(self.access_id)
        .execute(pool)
        .await?;
        let record: (i64,) = sqlx::query_as("SELECT last_insert_rowid()")
            .fetch_one(pool)
            .await?;
        Ok(record.0)
    }

    pub async fn find_by_id(pool: &SqlitePool, access_user_id: i64) -> anyhow::Result<AccessUser> {
        let access = sqlx::query_as::<_, AccessUser>(
            "SELECT id, user_id ,file_id , access_id FROM access_users WHERE id = $1",
        )
        .bind(access_user_id)
        .fetch_one(pool)
        .await?;
        Ok(access)
    }

    pub async fn find_by_user_id(
        pool: &SqlitePool,
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
        pool: &SqlitePool,
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

    pub async fn find_id(pool: &SqlitePool, username: &str, link: &str) -> anyhow::Result<i64> {
        let access: (i64,) =
            sqlx::query_as(
                "SELECT ac.id FROM access_users ac INNER JOIN users us ON us.id = ac.user_id INNER JOIN files fi ON fi.id = ac.file_id WHERE us.username = $1 AND fi.link = $2")
                .bind(username)
                .bind(link)
                .fetch_one(pool)
                .await?;
        Ok(access.0)
    }

    pub async fn is_user_access(
        pool: &SqlitePool,
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
        pool: &SqlitePool,
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
        pool: &SqlitePool,
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
        pool: &SqlitePool,
        access_user_id: i64,
        access_id: i64,
    ) -> anyhow::Result<u64> {
        let result = sqlx::query(r#"UPDATE access_users SET access_id = ?1 WHERE id = ?2"#)
            .bind(access_id)
            .bind(access_user_id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn update(&self, pool: &SqlitePool) -> anyhow::Result<u64> {
        let result = sqlx::query(
            r#"UPDATE access_users SET user_id = $1 ,file_id = $2 ,access_id = $3 WHERE id = $4"#,
        )
        .bind(self.user_id)
        .bind(self.file_id)
        .bind(self.access_id)
        .bind(self.id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn delete(pool: &SqlitePool, user_id: i64, file_id: i64) -> anyhow::Result<u64> {
        let result = sqlx::query(r#"DELETE FROM access_users WHERE user_id = $1 AND file_id = $2"#)
            .bind(user_id)
            .bind(file_id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn exist(pool: &SqlitePool, user_id: i64, file_id: i64) -> anyhow::Result<bool> {
        let id: (i64,) = sqlx::query_as(
            r#"SELECT EXISTS (SELECT id FROM access_users WHERE user_id = $1 OR file_id = $2)"#,
        )
        .bind(user_id)
        .bind(file_id)
        .fetch_one(pool)
        .await?;
        Ok(id.0 > 0)
    }
}
