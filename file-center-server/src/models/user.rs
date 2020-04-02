use crate::models::role::Role;
use crate::payloads::requests::LoginRequest;
use crate::utils::hash::Bcrypt;
use log::info;
use sqlx::decode::Decode;
use sqlx::encode::Encode;
use sqlx::prelude::*;
use sqlx::prelude::*;
use sqlx::sqlite::SqliteTypeInfo;
use sqlx::Row;
use sqlx::{FromRow, Sqlite, Type};
use sqlx::{Pool, SqliteConnection};
use anyhow::anyhow;

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
        let pass = Bcrypt::hash(password);
        Ok(User {
            id: 0,
            username: username.to_owned(),
            password: pass,
            email: email.to_owned(),
            role_id: role.id,
        })
    }

    pub async fn save(&self, pool: &Pool<SqliteConnection>) -> anyhow::Result<i64> {
        sqlx::query!(
            r#"INSERT INTO users (username,password,email,role_id) VALUES ($1,$2,$3,$4);"#,
            self.username,
            self.password,
            self.email,
            self.role_id,
        )
        .execute(pool)
        .await?;
        let record: (i64,) = sqlx::query_as("SELECT last_insert_rowid()")
            .fetch_one(pool)
            .await?;
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
            "SELECT username ,password ,email,role_id FROM users WHERE username = $1",
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
        // let mut stmt =
        //     conn.prepare("SELECT username FROM users WHERE username = ?1 OR email = ?2")?;
        // let result = stmt.exists(&[&self.username, &self.email]);
        // info!(
        //     "User {} with email {} exist => {:?}",
        //     self.username, self.email, result
        // );
        // return result;
        let id = sqlx::query!(
            r#"SELECT id FROM users WHERE username = ?1 OR email = ?2"#,
             self.username,self.email)
            .execute(pool)
            .await?;
        Ok(id>0)
    }

    pub async fn update(&self, pool: &Pool<SqliteConnection>) -> anyhow::Result<u64> {
        let id = sqlx::query!(
            r#"UPDATE users SET username = $1 ,password = $2 ,email = $3, role_id = $4 WHERE id = $5"#,
             self.username,self.password,self.email,self.role_id,&self.id)
            .execute(pool)
            .await?;
        Ok(id)
    }

    pub async fn delete(&self, pool: &Pool<SqliteConnection>) -> anyhow::Result<u64> {
        let id = sqlx::query!(r#"DELETE FROM users WHERE id = $1"#, self.id)
            .execute(pool)
            .await?;
        Ok(id)
    }

    pub async fn delete_by_email(pool: &Pool<SqliteConnection>, email: String) -> anyhow::Result<u64> {
        let id = sqlx::query!(r#"DELETE FROM users WHERE email = $1"#, email)
            .execute(pool)
            .await?;
        Ok(id)
    }

    pub async fn delete_by_username(
        pool: &Pool<SqliteConnection>,
        username: &str,
    ) -> anyhow::Result<u64> {
        let id = sqlx::query!(r#"DELETE FROM users WHERE username = $1"#, username)
            .execute(pool)
            .await?;
        Ok(id)
    }
}
