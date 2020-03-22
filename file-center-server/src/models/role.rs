use rusqlite::{params, Connection, Error, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub struct Role {
    pub id: i32,
    pub role_name: RoleName,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum RoleName {
    RoleAdmin,
    RoleUser,
}

impl fmt::Display for RoleName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RoleName::RoleAdmin => write!(f, "ROLE_ADMIN"),
            RoleName::RoleUser => write!(f, "ROLE_USER"),
        }
    }
}
impl From<String> for RoleName {
    fn from(input: String) -> RoleName {
        use RoleName::*;
        let input_uppercase = input.to_uppercase();
        if input_uppercase == RoleAdmin.to_string() {
            return RoleName::RoleAdmin;
        } else if input_uppercase == RoleAdmin.to_string() {
            return RoleName::RoleUser;
        } else {
            panic!("input invalid!");
        }
    }
}
impl Role {
    pub fn new(role_name: RoleName) -> Role {
        Role {
            id: role_name as i32,
            role_name,
        }
    }

    pub fn find_by_name(conn: &Connection, role_name: RoleName) -> Result<Role> {
        let mut stmt = conn.prepare("SELECT id, role_name FROM roles WHERE role_name = ?1")?;
        let role = stmt.query_map(&[role_name.to_string()], |row| {
            Ok(Role {
                id: row.get(0)?,
                role_name: row.get::<_, String>(1)?.into(),
            })
        })?;
        for r in role {
            return r;
        }
        Err(Error::InvalidQuery)
    }

    pub fn find_by_id(conn: &Connection, user_id: usize) -> Result<Role> {
        let mut stmt = conn.prepare("SELECT id, role_name FROM roles WHERE id = ?1")?;
        let role = stmt.query_map(&[user_id.to_string()], |row| {
            Ok(Role {
                id: row.get(0)?,
                role_name: row.get::<_, String>(1)?.into(),
            })
        })?;
        for r in role {
            return r;
        }
        Err(Error::InvalidQuery)
    }

    pub fn find_all(conn: &Connection) -> Result<Vec<Role>> {
        let mut stmt = conn.prepare("SELECT id,role_name FROM roles")?;
        let roles = stmt.query_map(params![], |row| {
            Ok(Role {
                id: row.get(0)?,
                role_name: row.get::<_, String>(1)?.into(),
            })
        })?;
        Ok(roles.into_iter().flat_map(|r| r).collect::<Vec<Role>>())
    }
}
