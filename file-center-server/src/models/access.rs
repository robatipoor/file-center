use rusqlite::{params, Connection, Error, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub struct Access {
    pub id: i32,
    pub access_type: AccessType,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum AccessType {
    Read,
    Write,
}

impl fmt::Display for AccessType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccessType::Read => write!(f, "READ"),
            AccessType::Write => write!(f, "WRITE"),
        }
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
    pub fn new(access_type: AccessType) -> Access {
        Access {
            id: access_type as i32,
            access_type,
        }
    }

    pub fn find_by_name(conn: &Connection, access_type: AccessType) -> Result<Access> {
        let mut stmt = conn.prepare("SELECT id, access_type FROM access WHERE access_type = ?1")?;
        let access = stmt.query_map(&[access_type.to_string()], |row| {
            Ok(Access {
                id: row.get(0)?,
                access_type: row.get::<_, String>(1)?.into(),
            })
        })?;
        for ac in access {
            return ac;
        }
        Err(Error::InvalidQuery)
    }

    pub fn find_by_id(conn: &Connection, access_id: usize) -> Result<Access> {
        let mut stmt = conn.prepare("SELECT id, access_type FROM access WHERE id = ?1")?;
        let access = stmt.query_map(&[access_id.to_string()], |row| {
            Ok(Access {
                id: row.get(0)?,
                access_type: row.get::<_, String>(1)?.into(),
            })
        })?;
        for ac in access {
            return ac;
        }
        Err(Error::InvalidQuery)
    }

    pub fn find_all(conn: &Connection) -> Result<Vec<Access>> {
        let mut stmt = conn.prepare("SELECT id,access_type FROM access")?;
        let all_access = stmt.query_map(params![], |row| {
            Ok(Access {
                id: row.get(0)?,
                access_type: row.get::<_, String>(1)?.into(),
            })
        })?;
        Ok(all_access
            .into_iter()
            .flat_map(|r| r)
            .collect::<Vec<Access>>())
    }
}
