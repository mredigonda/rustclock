use crate::time;
use rusqlite::Connection;

#[derive(Debug)]
pub struct Activity {
    pub id: i64,
    pub description: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub project_id: i64,
}

impl Activity {
    pub fn new(description: String, project_id: i64) -> Self {
        let now = time::Time::now();
        Activity {
            id: -1,
            description,
            start_time: now,
            end_time: None,
            project_id,
        }
    }

    pub fn get_report(&self) -> String {
        let duration_str = time::Time::elapsed_since(&self.start_time);
        format!(
            "Activity: {} | Project: {} | Duration: {}",
            self.description, "<project>", duration_str,
        )
    }

    pub fn end_now(&mut self) {
        self.end_time = Some(time::Time::now());
    }

    pub fn save(&mut self, storage: &mut Connection) {
        Self::initialize_storage(storage);
        let desc: &String = &self.description;
        let now = time::Time::now();
        println!("Project id: {}", self.project_id);

        if self.id == -1 {
            // In this case, we have a new activity that does not yet have an allocated id
            // We need to INSERT it into the storage
            storage
                .execute(
                    "INSERT INTO activity (description, start_time, project_id) VALUES (?1, ?2, ?3)",
                    (&desc, now, self.project_id),
                )
                .expect("RUSTCLOCK0002: There was a problem when saving an activity.");
            self.id = storage.last_insert_rowid();
        } else {
            // In this case, we have an already-existing activity
            // We need to UPDATE it in the storage
            storage
                .execute(
                    "UPDATE activity SET description=?1, start_time=?2, end_time=?3 WHERE id=?4;",
                    (&desc, &self.start_time, &self.end_time, &self.id),
                )
                .expect("RUSTCLOCK0003: There was a problem when updating an activity.");
        }
    }

    // TODO: update this to return an Result<Option<Activity>, ...> since it's possible to not have a current activity
    pub fn get_current(storage: &mut Connection) -> Result<Option<Activity>, rusqlite::Error> {
        Self::initialize_storage(storage);
        let mut statement = storage.prepare(
            "SELECT id, description, start_time, end_time, project_id FROM activity WHERE end_time IS NULL",
        )?;
        let mut activity_iter = statement.query_map([], |row| {
            Ok(Activity {
                id: row.get(0)?,
                description: row.get(1)?,
                start_time: row.get(2)?,
                end_time: row.get(3)?,
                project_id: row.get(4)?,
            })
        })?;
        let x = activity_iter.next();
        match x {
            Some(result) => Ok(Some(result?)),
            None => Ok(None),
        }
    }

    fn initialize_storage(storage: &mut Connection) {
        storage
            .execute(
                "CREATE TABLE IF NOT EXISTS activity (
                        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                        description VARCHAR(255) NOT NULL,
                        start_time VARCHAR(255) NOT NULL,
                        end_time VARCHAR(255),
                        project_id INT NOT NULL,
                        FOREIGN KEY (project_id) REFERENCES project(id)
                )",
                (),
            )
            .expect("RUSTCLOCK0001: There was a problem when initializing storage for Activity.");
    }
}
