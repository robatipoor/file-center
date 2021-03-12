pub mod access;
pub mod access_user;
pub mod file;
pub mod role;
pub mod user;

use crate::config::constants;
use crate::config::CONFIG;
use crate::utils::file::read_file;
use actix_web::web;
use log::info;
use sqlx::SqlitePool;
use std::fmt;

pub type DataPoolSqlite = web::Data<SqlitePool>;

pub struct DataBase {
    pub pool: SqlitePool,
}

pub enum DataDefinitionLanguageMode {
    UpdateSchema,
    CreateSchema,
    InsertData,
    DeleteData,
    DropAll,
    None,
}

impl fmt::Display for DataDefinitionLanguageMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DataDefinitionLanguageMode::*;
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

impl From<String> for DataDefinitionLanguageMode {
    fn from(input: String) -> Self {
        use DataDefinitionLanguageMode::*;
        let input_uppercase = input.to_uppercase();
        if input_uppercase == UpdateSchema.to_string() {
            DataDefinitionLanguageMode::UpdateSchema
        } else if input_uppercase == InsertData.to_string() {
            DataDefinitionLanguageMode::InsertData
        } else if input_uppercase == DeleteData.to_string() {
            DataDefinitionLanguageMode::DeleteData
        } else if input_uppercase == DropAll.to_string() {
            DataDefinitionLanguageMode::DropAll
        } else if input_uppercase == CreateSchema.to_string() {
            DataDefinitionLanguageMode::CreateSchema
        } else {
            DataDefinitionLanguageMode::None
        }
    }
}

impl DataDefinitionLanguageMode {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(CONFIG.database_mode.clone().into())
    }
}

impl DataBase {
    pub async fn new() -> anyhow::Result<DataBase> {
        Ok(DataBase {
            pool: DataBase::open_conn_pool().await?,
        })
    }

    pub async fn auto_migrate() -> anyhow::Result<DataBase> {
        use DataDefinitionLanguageMode::*;
        let ddl_mode = DataDefinitionLanguageMode::from_env()?;
        let database = DataBase::new().await?;
        match ddl_mode {
            UpdateSchema => database.update_schema().await?,
            InsertData => database.insert_data().await?,
            DeleteData => database.delete_data().await?,
            DropAll => database.drop_database().await?,
            CreateSchema => database.create_schema().await?,
            None => info!("None"),
        }
        Ok(database)
    }

    pub async fn get_conn_pool(self) -> SqlitePool {
        self.pool
    }

    async fn open_conn_pool() -> anyhow::Result<SqlitePool> {
        let pool = SqlitePool::connect(CONFIG.database_url.as_str()).await?;
        Ok(pool)
    }

    async fn update_schema(&self) -> anyhow::Result<()> {
        info!("Start Update DataBase");
        self.drop_database().await?;
        self.create_schema().await?;
        self.insert_data().await?;
        Ok(())
    }

    async fn create_schema(&self) -> anyhow::Result<()> {
        sqlx::query(&*read_file(constants::SCHEMA_SQL_FILE_PATH).await?)
            .execute(&self.pool)
            .await?;
        info!("Create Schema ");
        Ok(())
    }
    pub async fn insert_data(&self) -> anyhow::Result<()> {
        sqlx::query(&*read_file(constants::INSERT_SQL_FILE_PATH).await?)
            .execute(&self.pool)
            .await?;
        info!("Insert Data");
        Ok(())
    }

    pub async fn drop_database(&self) -> anyhow::Result<()> {
        sqlx::query(&*read_file(constants::DROP_SQL_FILE_PATH).await?)
            .execute(&self.pool)
            .await?;
        info!("Drop Tables");
        Ok(())
    }

    pub async fn delete_data(&self) -> anyhow::Result<()> {
        sqlx::query(&*read_file(constants::DELETE_SQL_FILE_PATH).await?)
            .execute(&self.pool)
            .await?;
        info!("Delete Data");
        Ok(())
    }
}
