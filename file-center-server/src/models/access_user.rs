use log::info;
use rusqlite::{params, Connection, Error, Result};

pub struct AccessUser {
    pub id: i32,
    pub user_id: i32,
    pub file_id: i32,
    pub access_id: i32,
}

impl AccessUser {
    pub fn new(user_id: i32, file_id: i32, access_id: i32) -> AccessUser {
        AccessUser {
            id: 0,
            user_id,
            file_id,
            access_id,
        }
    }

    pub fn save(&self, conn: &Connection) -> Result<usize> {
        let id = conn.execute(
            "INSERT INTO access_users (user_id , file_id , access_id) values (?1, ?2 ,?3)",
            &[
                &self.user_id.to_string(),
                &self.file_id.to_string(),
                &self.access_id.to_string(),
            ],
        )?;
        Ok(id)
    }

    pub fn find_by_id(conn: &Connection, access_user_id: i32) -> Result<AccessUser> {
        let mut stmt = conn
            .prepare("SELECT id, user_id ,file_id , access_id FROM access_users WHERE id = ?1")?;
        let access_users = stmt.query_map(&[access_user_id.to_string()], |row| {
            Ok(AccessUser {
                id: row.get(0)?,
                user_id: row.get(1)?,
                file_id: row.get(2)?,
                access_id: row.get(3)?,
            })
        })?;
        for access_user in access_users {
            return access_user;
        }
        Err(Error::InvalidQuery)
    }

    pub fn find_by_user_id(conn: &Connection, user_id: i32) -> Result<Vec<AccessUser>> {
        let mut stmt = conn.prepare(
            "SELECT id, user_id ,file_id , access_id FROM access_users WHERE user_id = ?1",
        )?;
        let access_users = stmt.query_map(&[user_id.to_string()], |row| {
            Ok(AccessUser {
                id: row.get(0)?,
                user_id: row.get(1)?,
                file_id: row.get(2)?,
                access_id: row.get(3)?,
            })
        })?;
        Ok(access_users
            .into_iter()
            .flat_map(|r| r)
            .collect::<Vec<AccessUser>>())
    }

    pub fn find_by_file_id(conn: &Connection, file_id: i32) -> Result<Vec<AccessUser>> {
        let mut stmt = conn.prepare(
            "SELECT id, user_id ,file_id , access_id FROM access_users WHERE file_id = ?1",
        )?;
        let access_users = stmt.query_map(&[file_id.to_string()], |row| {
            Ok(AccessUser {
                id: row.get(0)?,
                user_id: row.get(1)?,
                file_id: row.get(2)?,
                access_id: row.get(3)?,
            })
        })?;
        Ok(access_users
            .into_iter()
            .flat_map(|r| r)
            .collect::<Vec<AccessUser>>())
    }

    pub fn find_id(conn: &Connection, user_id: i32, file_id: i32) -> Result<i32> {
        let mut stmt =
            conn.prepare("SELECT id FROM access_users WHERE user_id = ?1 AND file_id = ?2")?;
        let access_users = stmt.query_map(&[user_id.to_string()], |row| Ok(row.get(0)?))?;
        for access_user in access_users {
            return access_user;
        }
        Err(Error::InvalidQuery)
    }

    pub fn is_user_access(
        conn: &Connection,
        user_id: i32,
        file_id: i32,
        access_id: i32,
    ) -> Result<bool> {
        let mut stmt = conn.prepare(
            "SELECT id FROM access_users WHERE user_id = ?1 AND file_id = ?2 AND access_id = ?3",
        )?;
        let result = stmt.exists(&[user_id, file_id, access_id]);
        info!(
            "User {} with file {} access {} => {:?}",
            user_id, file_id, access_id, result
        );
        return result;
    }

    // pub fn exist(&self, conn: &Connection) -> Result<bool> {
    //     let mut stmt =
    //         conn.prepare("SELECT username FROM users WHERE username = ?1 OR email = ?2")?;
    //     let result = stmt.exists(&[&self.username, &self.email]);
    //     info!(
    //         "User {} with email {} exist => {:?}",
    //         self.username, self.email, result
    //     );
    //     return result;
    // }

    pub fn update_access(conn: &Connection, access_user_id: i32, access_id: i32) -> Result<usize> {
        let id = conn.execute(
            "UPDATE FROM files SET access_id = ?1 WHERE id = ?2",
            &[access_id, access_user_id],
        )?;
        Ok(id)
    }

    pub fn update(&self, conn: &Connection) -> Result<usize> {
        let id = conn.execute(
            "UPDATE FROM files SET user_id = ?1 file_id = ,?2 access_id = ?3",
            &[
                &self.user_id.to_string(),
                &self.file_id.to_string(),
                &self.access_id.to_string(),
            ],
        )?;
        Ok(id)
    }

    pub fn delete(&self, conn: &Connection) -> Result<usize> {
        let id = conn.execute("DELETE FROM access_users WHERE id = ?1", &[&self.id])?;
        Ok(id)
    }
}
