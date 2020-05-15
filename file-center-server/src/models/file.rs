use crate::payloads::responses::FileResponse;
use log::info;
use sqlx::prelude::*;
use sqlx::{Pool, SqliteConnection};
#[derive(sqlx::FromRow, Debug)]
pub struct File {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub link: String,
    pub user_id: i64,
}

impl File {
    pub async fn new(name: &str, path: &str, link: &str, user_id: i64) -> anyhow::Result<File> {
        Ok(File {
            id: 0,
            name: name.to_owned(),
            path: path.to_owned(),
            link: link.to_owned(),
            user_id,
        })
    }
    pub async fn save(&self, pool: &Pool<SqliteConnection>) -> anyhow::Result<i64> {
        println!("{} {} {}", self.name, self.path, self.user_id);
        sqlx::query(r#"INSERT INTO files (name, path ,link,user_id) VALUES ($1,$2,$3,$4)"#)
            .bind(self.name.as_str())
            .bind(self.path.as_str())
            .bind(self.link.as_str())
            .bind(self.user_id)
            .execute(pool)
            .await?;
        let record: (i64,) = sqlx::query_as("SELECT last_insert_rowid()")
            .fetch_one(pool)
            .await?;
        info!("");
        Ok(record.0)
    }

    pub async fn find_id(pool: &Pool<SqliteConnection>, link: &str) -> anyhow::Result<i64> {
        let id: (i64,) = sqlx::query_as("SELECT id FROM files WHERE link = $1")
            .bind(link)
            .fetch_one(pool)
            .await?;
        info!("");
        Ok(id.0)
    }

    pub async fn find_by_id(pool: &Pool<SqliteConnection>, file_id: i64) -> anyhow::Result<File> {
        let file = sqlx::query_as::<_, File>(
            "SELECT id, name ,path , link, user_id FROM files WHERE id = $1",
        )
        .bind(file_id)
        .fetch_one(pool)
        .await?;
        info!("");
        Ok(file)
    }

    pub async fn find_path_by_link(
        pool: &Pool<SqliteConnection>,
        link: &str,
    ) -> anyhow::Result<String> {
        let path: (String,) = sqlx::query_as("SELECT path FROM files WHERE link = $1")
            .bind(link)
            .fetch_one(pool)
            .await?;
        info!("");
        Ok(path.0)
    }

    pub async fn find_by_link(pool: &Pool<SqliteConnection>, link: &str) -> anyhow::Result<File> {
        let file = sqlx::query_as::<_, File>(
            "SELECT id, name, path, link, user_id FROM files WHERE link = $1",
        )
        .bind(link)
        .fetch_one(pool)
        .await?;
        info!("");
        Ok(file)
    }

    pub async fn find_by_user(
        pool: &Pool<SqliteConnection>,
        user_id: i64,
    ) -> anyhow::Result<Vec<File>> {
        let files = sqlx::query_as::<_, File>(
            "SELECT id, name, path ,link ,user_id FROM files WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;
        info!("");
        Ok(files)
    }

    pub async fn find_all_link_files(
        pool: &Pool<SqliteConnection>,
        user_id: i64,
    ) -> anyhow::Result<Vec<FileResponse>> {
        let links: Vec<FileResponse> =
            sqlx::query_as("SELECT name,link FROM files WHERE user_id = $1")
                .bind(user_id)
                .fetch_all(pool)
                .await?;
        info!("find all link by user id");
        Ok(links.into_iter().collect())
    }

    pub async fn is_owner(
        pool: &Pool<SqliteConnection>,
        link: &str,
        user_id: i64,
    ) -> anyhow::Result<bool> {
        let id: (i64,) = sqlx::query_as(
            r#"SELECT EXISTS (SELECT id FROM files WHERE user_id = $1 AND link = $2)"#,
        )
        .bind(user_id)
        .bind(link)
        .fetch_one(pool)
        .await?;
        info!("");
        Ok(id.0 > 0)
    }

    pub async fn update(&self, pool: &Pool<SqliteConnection>) -> anyhow::Result<u64> {
        let row_affected = sqlx::query(
            r#"UPDATE files SET name = $1 ,path = $2 ,link = $3 ,user_id = $4 WHERE id = $5"#,
        )
        .bind(self.name.as_str())
        .bind(self.path.as_str())
        .bind(self.link.as_str())
        .bind(self.user_id)
        .bind(self.id)
        .execute(pool)
        .await?;
        // info!("");
        Ok(row_affected)
    }

    pub async fn delete(&self, pool: &Pool<SqliteConnection>) -> anyhow::Result<u64> {
        let row_affected = sqlx::query(r#"DELETE FROM files WHERE id = $1"#)
            .bind(self.id)
            .execute(pool)
            .await?;
        info!("");
        Ok(row_affected)
    }

    pub async fn delete_by_link(pool: &Pool<SqliteConnection>, link: &str) -> anyhow::Result<u64> {
        let row_affected = sqlx::query(r#"DELETE FROM files WHERE link = $1"#)
            .bind(link)
            .execute(pool)
            .await?;
        info!("");
        Ok(row_affected)
    }

    pub async fn delete_by_path(pool: &Pool<SqliteConnection>, path: &str) -> anyhow::Result<u64> {
        let row_affected = sqlx::query(r#"DELETE FROM files WHERE path = $1"#)
            .bind(path)
            .execute(pool)
            .await?;
        info!("");
        Ok(row_affected)
    }
}
