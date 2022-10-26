use crate::time;
use rusqlite::Connection;

#[derive(Debug)]
pub struct Activity {
    pub id: i32,
    pub description: String,
    pub start_time: String,
    pub end_time: Option<String>,
}

impl Activity {
    pub fn new(description: String) -> Self {
        let now = time::Time::now();
        Activity {
            id: -1,
            description,
            start_time: now,
            end_time: None,
        }
    }

    pub fn get_current(storage: &mut Connection) -> Result<Activity, rusqlite::Error> {
        Self::initialize_storage(storage);
        let mut statement = storage.prepare(
            "SELECT id, description, start_time, end_time FROM activity WHERE end_time IS NULL",
        )?;
        let mut activity_iter = statement.query_map([], |row| {
            Ok(Activity {
                id: row.get(0)?,
                description: row.get(1)?,
                start_time: row.get(2)?,
                end_time: row.get(3)?,
            })
        })?;
        let x = activity_iter.next().unwrap();
        x
    }

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
