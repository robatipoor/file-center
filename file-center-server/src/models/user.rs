use rusqlite::{params, Connection, Error, Result};

use crate::models::role::Role;
use crate::payloads::requests::LoginRequest;
use crate::utils::hash::Bcrypt;
use log::info;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub role_id: i32,
}

impl User {
    pub fn new(username: &str, password: &str, email: &str, role: Role) -> User {
        let pass = Bcrypt::hash(password);
        User {
            id: 0,
            username: username.to_owned(),
            password: pass,
            email: email.to_owned(),
            role_id: role.id,
        }
    }
    pub fn save(&self, conn: &Connection) -> Result<usize> {
        let id = conn.execute(
            "INSERT INTO users (username, password ,email,role_id) values (?1, ?2 ,?3,?4)",
            &[
                &self.username,
                &self.password,
                &self.email,
                &self.role_id.to_string(),
            ],
        )?;
        Ok(id)
    }

    pub fn find_id(conn: &Connection, username: &str) -> Result<i32> {
        let mut stmt =
            conn.prepare("SELECT id FROM users WHERE username = ?1")?;
        let users = stmt.query_map(&[username], |row| {
            Ok(row.get(0)?)
        })?;
        for user in users {
            return user;
        }
        Err(Error::InvalidQuery)
    }

    pub fn find_by_id(conn: &Connection, user_id: usize) -> Result<User> {
        let mut stmt =
            conn.prepare("SELECT id, username, password, email, role_id FROM users WHERE id = ?1")?;
        let users = stmt.query_map(&[user_id.to_string()], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
                email: row.get(3)?,
                role_id: row.get(4)?,
            })
        })?;
        for user in users {
            return user;
        }
        Err(Error::InvalidQuery)
    }

    pub fn verify(conn: &Connection, login: LoginRequest) -> std::result::Result<User, String> {
        let user = User::find_by_username(conn, &*login.username);
        if let Ok(u) = user {
            if Bcrypt::verify(&*login.password, &*u.password) {
                Ok(u)
            } else {
                Err("User Not Valid!".to_owned())
            }
        } else {
            Err("User Not Exist!".to_owned())
        }
    }

    pub fn find_by_email(conn: &Connection, email: &str) -> Result<User> {
        let mut stmt = conn
            .prepare("SELECT id, username, password, email, role_id FROM users WHERE email = ?1")?;
        let users = stmt.query_map(&[email], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
                email: row.get(3)?,
                role_id: row.get(4)?,
            })
        })?;
        for user in users {
            return user;
        }
        Err(Error::InvalidQuery)
    }

    pub fn find_by_username(conn: &Connection, username: &str) -> Result<User> {
        let mut stmt = conn.prepare(
            "SELECT id, username, password, email, role_id FROM users WHERE username = ?1",
        )?;
        let users = stmt.query_map(&[username], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
                email: row.get(3)?,
                role_id: row.get(4)?,
            })
        })?;
        for user in users {
            return user;
        }
        Err(Error::InvalidQuery)
    }

    pub fn find_all(conn: &Connection) -> Result<Vec<User>> {
        let mut stmt = conn.prepare("SELECT id, username, password ,email ,role_id FROM users")?;
        let users = stmt.query_map(params![], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
                email: row.get(3)?,
                role_id: row.get(4)?,
            })
        })?;
        Ok(users.into_iter().flat_map(|r| r).collect::<Vec<User>>())
    }

    pub fn exist(&self, conn: &Connection) -> Result<bool> {
        let mut stmt =
            conn.prepare("SELECT username FROM users WHERE username = ?1 OR email = ?2")?;
        let result = stmt.exists(&[&self.username, &self.email]);
        info!(
            "User {} with email {} exist => {:?}",
            self.username, self.email, result
        );
        return result;
    }

    pub fn update(&self, conn: &Connection) -> Result<usize> {
        let id = conn.execute("UPDATE FROM users SET username = ?1 password = ,?2 email = ?3, role_name = ?4 WHERE id = ?5", &[&self.username,&self.password,&self.email,&self.role_id.to_string(),&self.id.to_string()])?;
        Ok(id)
    }

    pub fn delete(&self, conn: &Connection) -> Result<usize> {
        let id = conn.execute("DELETE FROM users WHERE id = ?1", &[&self.id])?;
        Ok(id)
    }

    pub fn delete_by_email(conn: &Connection, email: String) -> Result<usize> {
        let id = conn.execute("DELETE FROM users WHERE email = ?1", &[email])?;
        Ok(id)
    }

    pub fn delete_by_username(conn: &Connection, username: String) -> Result<usize> {
        let id = conn.execute("DELETE FROM users WHERE username = ?1", &[username])?;
        Ok(id)
    }
}
