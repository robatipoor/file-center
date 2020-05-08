use crate::models::role::Role;
use crate::payloads::requests::LoginRequest;
use crate::utils::hash::Bcrypt;
use anyhow::anyhow;
use log::info;
use sqlx::prelude::*;
use sqlx::{Pool, SqliteConnection};

#[derive(Debug)]
pub struct UserAuth {
    pub id: i64,
    pub role_id: i64,
}

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub email: String,
    pub role_id: i64,
}

impl User {
    pub async fn new(
        username: &str,
        password: &str,
        email: &str,
        role: Role,
    ) -> anyhow::Result<User> {
        let hashed_pass = Bcrypt::hash(password);
        Ok(User {
            id: 0,
            username: username.to_owned(),
            password: hashed_pass,
            email: email.to_owned(),
            role_id: role.id,
        })
    }

    pub async fn save(&self, pool: &Pool<SqliteConnection>) -> anyhow::Result<i64> {
        sqlx::query(r#"INSERT INTO users (username,password,email,role_id) VALUES ($1,$2,$3,$4);"#)
            .bind(self.username.as_str())
            .bind(self.password.as_str())
            .bind(self.email.as_str())
            .bind(self.role_id)
            .execute(pool)
            .await?;
        let record: (i64,) = sqlx::query_as("SELECT last_insert_rowid()")
            .fetch_one(pool)
            .await?;
        info!("");
        Ok(record.0)
    }

    pub async fn find_id(pool: &Pool<SqliteConnection>, username: &str) -> anyhow::Result<i64> {
        let id: (i64,) = sqlx::query_as("SELECT id FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(pool)
            .await?;
        Ok(id.0)
    }

    pub async fn find_by_id(pool: &Pool<SqliteConnection>, user_id: i64) -> anyhow::Result<User> {
        let user = sqlx::query_as::<_, User>(
            "SELECT username ,password ,email,role_id FROM users WHERE id = $1",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    pub async fn find_by_email(pool: &Pool<SqliteConnection>, email: &str) -> anyhow::Result<User> {
        let user = sqlx::query_as::<_, User>(
            "SELECT username ,password ,email,role_id FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    pub async fn find_by_username(
        pool: &Pool<SqliteConnection>,
        username: &str,
    ) -> anyhow::Result<User> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id ,username ,password ,email ,role_id FROM users WHERE username = $1",
        )
        .bind(username)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    pub async fn find_all(pool: &Pool<SqliteConnection>) -> anyhow::Result<Vec<User>> {
        let users = sqlx::query_as::<_, User>(
            "SELECT username ,password ,email,role_id FROM users WHERE username = $1",
        )
        .fetch_all(pool)
        .await?;
        Ok(users)
    }

    pub async fn verify(
        pool: &Pool<SqliteConnection>,
        login: LoginRequest,
    ) -> anyhow::Result<User> {
        let user = User::find_by_username(pool, &*login.username).await?;
        if Bcrypt::verify(&*login.password, &*user.password) {
            Ok(user)
        } else {
            Err(anyhow!("User Not Verify"))
        }
    }

    pub async fn exist(&self, pool: &Pool<SqliteConnection>) -> anyhow::Result<bool> {
        let id: (i64,) = sqlx::query_as(
            r#"SELECT EXISTS (SELECT id FROM users WHERE username = $1 OR email = $2)"#,
        )
        .bind(&self.username)
        .bind(&self.email)
        .fetch_one(pool)
        .await?;
        Ok(id.0 > 0)
    }

    pub async fn update(&self, pool: &Pool<SqliteConnection>) -> anyhow::Result<u64> {
        let row_affected = sqlx::query(
            r#"UPDATE users SET username = $1 ,password = $2 ,email = $3, role_id = $4 WHERE id = $5"#,
            )
             .bind(self.username.as_str())
             .bind(self.password.as_str())
             .bind(self.email.as_str())
             .bind(self.role_id)
             .bind(self.id)
            .execute(pool)
            .await?;
        Ok(row_affected)
    }

    pub async fn delete(&self, pool: &Pool<SqliteConnection>) -> anyhow::Result<u64> {
        let row_affected = sqlx::query(r#"DELETE FROM users WHERE id = $1"#)
            .bind(self.id)
            .execute(pool)
            .await?;
        Ok(row_affected)
    }

    pub async fn delete_by_email(
        pool: &Pool<SqliteConnection>,
        email: String,
    ) -> anyhow::Result<u64> {
        let row_affected = sqlx::query(r#"DELETE FROM users WHERE email = $1"#)
            .bind(email)
            .execute(pool)
            .await?;
        Ok(row_affected)
    }

    pub async fn delete_by_username(
        pool: &Pool<SqliteConnection>,
        username: &str,
    ) -> anyhow::Result<u64> {
        let row_affected = sqlx::query(r#"DELETE FROM users WHERE username = $1"#)
            .bind(username)
            .execute(pool)
            .await?;
        Ok(row_affected)
    }
}
