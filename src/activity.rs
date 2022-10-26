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
        storage
            .execute("INSERT INTO activity (description) VALUES (?1)", (&desc,))
            .expect("RUSCLOCK0002: There was a problem when saving an activity.");
    }

    fn initialize_storage(storage: &mut Connection) {
        storage
            .execute(
                "CREATE TABLE IF NOT EXISTS activity (
                        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                        description VARCHAR(255) NOT NULL
                )",
                (),
            )
            .expect("RUSTCLOCK0001: There was a problem when initializing storage for Activity.");
    }
}
