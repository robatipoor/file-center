pub mod access;
pub mod access_user;
pub mod file;
pub mod role;
pub mod user;

use crate::utils::file::read_file;
use sqlx::prelude::*;
use sqlx::{Pool, SqliteConnection, SqlitePool};
use std::env;
use std::fmt;

pub enum DatabaseMode {
    UpdateSchema,
    CreateSchema,
    InsertData,
    DeleteData,
    DropAll,
    None,
}

impl fmt::Display for DatabaseMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DatabaseMode::*;
        let name = match self {
            UpdateSchema => "UPDATE",
            InsertData => "INSERT",
            DeleteData => "DELETE",
            DropAll => "DROP",
            CreateSchema => "CREATE",
            None => "NONE",
        };
        write!(f, "{}", name)
    }
}

impl From<String> for DatabaseMode {
    fn from(input: String) -> Self {
        use DatabaseMode::*;
        let input_uppercase = input.to_uppercase();
        if input_uppercase == UpdateSchema.to_string() {
            DatabaseMode::UpdateSchema
        } else if input_uppercase == InsertData.to_string() {
            DatabaseMode::InsertData
        } else if input_uppercase == DeleteData.to_string() {
            DatabaseMode::DeleteData
        } else if input_uppercase == DropAll.to_string() {
            DatabaseMode::DropAll
        } else if input_uppercase == CreateSchema.to_string() {
            DatabaseMode::CreateSchema
        } else {
            DatabaseMode::None
        }
    }
}

impl DatabaseMode {
    pub fn from_env() -> anyhow::Result<Self> {
        let m = std::env::var("DATABASE_MODE")?;
        Ok(m.into())
    }
}

pub struct DataBase {
    pub pool: Pool<SqliteConnection>,
}

impl DataBase {
    pub async fn new() -> anyhow::Result<DataBase> {
        Ok(DataBase {
            pool: DataBase::open_conn_pool().await?,
        })
    }

    pub async fn migrate() -> anyhow::Result<DataBase> {
        use DatabaseMode::*;
        let mod_db = DatabaseMode::from_env()?;
        let db = DataBase::new().await?;
        match mod_db {
            UpdateSchema => db.update_schema().await?,
            InsertData => db.insert_data().await?,
            DeleteData => db.delete_data().await?,
            DropAll => db.drop_database().await?,
            CreateSchema => db.create_schema().await?,
            None => println!("Noting ..."),
        }
        Ok(db)
    }

    pub async fn get_conn_pool(self) -> Pool<SqliteConnection> {
        return self.pool;
    }

    async fn open_conn_pool() -> anyhow::Result<Pool<SqliteConnection>> {
        let pool = SqlitePool::new(&*env::var("DATABASE_URL")?).await?;
        Ok(pool)
    }

    async fn update_schema(&self) -> anyhow::Result<()> {
        println!("Update DataBase ...");
        self.drop_database().await?;
        self.create_schema().await?;
        self.insert_data().await?;
        Ok(())
    }

    async fn create_schema(&self) -> anyhow::Result<()> {
        sqlx::query(&*read_file("sql/schema.sql")?)
            .execute(&self.pool)
            .await?;
        println!("Create Schema ...");
        Ok(())
    }
    pub async fn insert_data(&self) -> anyhow::Result<()> {
        sqlx::query(&*read_file("sql/insert.sql")?)
            .execute(&self.pool)
            .await?;
        println!("Insert Data ...");
        Ok(())
    }

    pub async fn drop_database(&self) -> anyhow::Result<()> {
        sqlx::query(&*read_file("sql/drop.sql")?)
            .execute(&self.pool)
            .await?;
        println!("Drop Table ...");
        Ok(())
    }

    pub async fn delete_data(&self) -> anyhow::Result<()> {
        sqlx::query(&*read_file("sql/delete.sql")?)
            .execute(&self.pool)
            .await?;
        println!("Delete Data ...");
        Ok(())
    }
}
