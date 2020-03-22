pub mod access;
pub mod access_user;
pub mod file;
pub mod role;
pub mod user;

extern crate rusqlite;

use crate::utils::file::read_file;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, Result};
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
    pub fn from_env() -> std::result::Result<DatabaseMode, std::env::VarError> {
        let m = std::env::var("DATABASE_MODE")?;
        Ok(m.into())
    }
}

pub struct DataBase {
    pub connection: Pool<SqliteConnectionManager>,
}

impl DataBase {
    pub fn new() -> DataBase {
        DataBase {
            connection: DataBase::get_connection_pool(),
        }
    }
    pub fn migrate() -> Result<DataBase> {
        use DatabaseMode::*;
        let mod_db = DatabaseMode::from_env().unwrap();
        let db = DataBase {
            connection: DataBase::get_connection_pool(),
        };
        match mod_db {
            UpdateSchema => db.update_schema()?,
            InsertData => db.insert_data()?,
            DeleteData => db.delete_data()?,
            DropAll => db.drop_database()?,
            CreateSchema => db.create_schema()?,
            None => println!("Noting ..."),
        }
        Ok(db)
    }

    pub fn get_connection(self) -> Pool<SqliteConnectionManager> {
        return self.connection;
    }

    fn open_connection() -> Result<Connection> {
        Connection::open(std::env::var("DATABASE_URL").unwrap())
    }

    fn get_connection_pool() -> Pool<SqliteConnectionManager> {
        let manager = SqliteConnectionManager::file(std::env::var("DATABASE_URL").unwrap());
        let pool = r2d2::Pool::new(manager).unwrap();
        pool
    }

    fn update_schema(&self) -> Result<()> {
        println!("Update DataBase ...");
        self.drop_database()?;
        self.create_schema()?;
        self.insert_data()?;
        Ok(())
    }
    fn create_schema(&self) -> Result<()> {
        self.connection
            .get()
            .unwrap()
            .execute_batch(&*read_file("sql/schema.sql").unwrap())?;
        println!("Create Schema ");
        Ok(())
    }
    pub fn insert_data(&self) -> Result<()> {
        self.connection
            .get()
            .unwrap()
            .execute_batch(&*read_file("sql/insert.sql").unwrap())?;
        println!("Insert Data ");
        Ok(())
    }
    pub fn drop_database(&self) -> Result<()> {
        self.connection
            .get()
            .unwrap()
            .execute_batch(&*read_file("sql/drop.sql").unwrap())?;
        println!("Drop DataBase ");
        Ok(())
    }

    pub fn delete_data(&self) -> Result<()> {
        self.connection
            .get()
            .unwrap()
            .execute_batch(&*read_file("sql/delete.sql").unwrap())?;
        println!("Delete Data ");
        Ok(())
    }
}
