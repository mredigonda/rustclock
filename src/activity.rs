use crate::time;
use rusqlite::Connection;

pub struct Activity {
    pub id: i32,
    pub description: String,
}

impl Activity {
    pub fn new(description: String) -> Self {
        Activity {
            id: -1,
            description,
        }
    }

    // pub fn get_current() -> Option<Self> {
    //     None
    // }

    pub fn save(&self, storage: &mut Connection) {
        Self::initialize_storage(storage);
        let desc: &String = &self.description;
        let now = time::Time::now();

        storage
            .execute(
                "INSERT INTO activity (description, start_time) VALUES (?1, ?2)",
                (&desc, now),
            )
            .expect("RUSCLOCK0002: There was a problem when saving an activity.");
    }

    fn initialize_storage(storage: &mut Connection) {
        storage
            .execute(
                "CREATE TABLE IF NOT EXISTS activity (
                        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                        description VARCHAR(255) NOT NULL,
                        start_time VARCHAR(255) NOT NULL,
                        end_time VARCHAR(255)
                )",
                (),
            )
            .expect("RUSTCLOCK0001: There was a problem when initializing storage for Activity.");
    }
}
