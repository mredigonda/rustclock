use rusqlite::Connection;

use crate::constants;

pub struct Project {
    pub id: i32,
    pub name: String,
}

impl Project {
    pub fn new(name: String) -> Self {
        Project { id: -1, name }
    }

    pub fn get_projects() -> Vec<Project> {
        vec![Project {
            id: -1,
            name: String::from(constants::CREATE_NEW_PROJECT_STRING),
        }]
    }

    pub fn save(&self, storage: &mut Connection) {
        Self::initialize_storage(storage);
        storage
            .execute(
                "INSERT INTO project (name) VALUES (?1)
        ",
                (&self.name,),
            )
            .expect("RUSTCLOCK0005: There was a problem when saving a project.");

        // TODO: maybe return id...
    }

    fn initialize_storage(storage: &mut Connection) {
        storage
            .execute(
                "CREATE TABLE IF NOT EXISTS project (
                        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                        name VARCHAR(255) NOT NULL
                )",
                (),
            )
            .expect("RUSTCLOCK0004: There was a problem when initializing storage for Project.");
    }
}
