use log::info;
use rusqlite::{params, Connection, Error, Result};
pub struct File {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub link: String,
    pub user_id: i32,
}

impl File {
    pub fn new(name: &str, path: &str, link: &str, user_id: i32) -> File {
        File {
            id: 0,
            name: name.to_owned(),
            path: path.to_owned(),
            link: link.to_owned(),
            user_id,
        }
    }
    pub fn save(&self, conn: &Connection) -> Result<usize> {
        let id = conn.execute(
            "INSERT INTO files (name, path ,link,user_id) values (?1, ?2 ,?3,?4)",
            &[
                &self.name,
                &self.path,
                &self.link,
                &self.user_id.to_string(),
            ],
        )?;
        Ok(id)
    }

    pub fn find_id(conn: &Connection, link: &str) -> Result<i32> {
        let mut stmt = conn.prepare("SELECT id FROM files WHERE link = ?1")?;
        let files = stmt.query_map(&[link], |row| Ok(row.get(0)?))?;
        for file in files {
            return file;
        }
        Err(Error::InvalidQuery)
    }

    pub fn find_by_id(conn: &Connection, file_id: i32) -> Result<File> {
        let mut stmt =
            conn.prepare("SELECT id, name ,path , link, user_id FROM files WHERE id = ?1")?;
        let files = stmt.query_map(&[file_id.to_string()], |row| {
            Ok(File {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                link: row.get(3)?,
                user_id: row.get(4)?,
            })
        })?;
        for file in files {
            return file;
        }
        Err(Error::InvalidQuery)
    }

    pub fn find_by_path(conn: &Connection, path: &str) -> Result<File> {
        let mut stmt =
            conn.prepare("SELECT id, name, path, link, user_id FROM files WHERE path = ?1")?;
        let files = stmt.query_map(&[path], |row| {
            Ok(File {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                link: row.get(3)?,
                user_id: row.get(4)?,
            })
        })?;
        for file in files {
            return file;
        }
        Err(Error::InvalidQuery)
    }

    pub fn find_by_link(conn: &Connection, link: &str) -> Result<File> {
        let mut stmt =
            conn.prepare("SELECT id, name, path, link, user_id FROM files WHERE link = ?1")?;
        let files = stmt.query_map(&[link], |row| {
            Ok(File {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                link: row.get(3)?,
                user_id: row.get(4)?,
            })
        })?;
        for file in files {
            return file;
        }
        Err(Error::InvalidQuery)
    }

    pub fn find_by_user(conn: &Connection, user_id: i32) -> Result<Vec<File>> {
        let mut stmt =
            conn.prepare("SELECT id, name, path ,link ,user_id FROM files WHERE user_id =?1")?;
        let files = stmt.query_map(&[user_id], |row| {
            Ok(File {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                link: row.get(3)?,
                user_id: row.get(4)?,
            })
        })?;
        Ok(files.into_iter().flat_map(|r| r).collect::<Vec<File>>())
    }

    pub fn find_all(conn: &Connection) -> Result<Vec<File>> {
        let mut stmt = conn.prepare("SELECT id, name, path ,link ,user_id FROM files")?;
        let files = stmt.query_map(params![], |row| {
            Ok(File {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                link: row.get(3)?,
                user_id: row.get(4)?,
            })
        })?;
        Ok(files.into_iter().flat_map(|r| r).collect::<Vec<File>>())
    }

    pub fn is_owner(conn: &Connection, file_id: i32, user_id: i32) -> Result<bool> {
        let mut stmt = conn.prepare("SELECT id FROM files WHERE id = ?1 AND user_id = ?2")?;
        let result = stmt.exists(&[file_id, user_id]);
        info!(
            "User id {} file id {} is owner => {:?}",
            user_id, file_id, result
        );
        return result;
    }

    pub fn is_owner_by_link(conn: &Connection, link: &str, user_id: i32) -> Result<bool> {
        let mut stmt = conn.prepare("SELECT id FROM files WHERE link = ?1 AND user_id = ?2")?;
        let result = stmt.exists(&[link.to_owned(), user_id.to_string()]);
        info!(
            "User id {} link file {} is owner => {:?}",
            user_id, link, result
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

    pub fn update(&self, conn: &Connection) -> Result<usize> {
        let id = conn.execute(
            "UPDATE FROM files SET name = ?1 path = ,?2 link = ?3, user_id = ?4 WHERE id = ?5",
            &[
                &self.name,
                &self.path,
                &self.link,
                &self.user_id.to_string(),
                &self.id.to_string(),
            ],
        )?;
        Ok(id)
    }

    pub fn delete(&self, conn: &Connection) -> Result<usize> {
        let id = conn.execute("DELETE FROM files WHERE id = ?1", &[&self.id])?;
        Ok(id)
    }

    pub fn delete_by_link(conn: &Connection, link: String) -> Result<usize> {
        let id = conn.execute("DELETE FROM files WHERE link = ?1", &[link])?;
        Ok(id)
    }

    pub fn delete_by_path(conn: &Connection, path: String) -> Result<usize> {
        let id = conn.execute("DELETE FROM files WHERE path = ?1", &[path])?;
        Ok(id)
    }
}
