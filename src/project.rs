use rusqlite::Connection;

use crate::constants;

#[derive(Debug)]
pub struct Project {
    pub id: i64,
    pub name: String,
}

impl Project {
    pub fn new(name: String) -> Self {
        Project { id: -1, name }
    }

    pub fn get_projects(storage: &mut Connection) -> Result<Vec<Project>, rusqlite::Error> {
        Self::initialize_storage(storage);
        let mut statement = storage.prepare("SELECT id, name FROM project")?;
        let project_iter = statement.query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;
        // let : Vec<Result<Project, rusqlite::Error>> = project_iter.collect();
        let mut projects: Vec<Project> = Vec::new();
        for project in project_iter {
            let actual_project = project.unwrap();
            projects.push(actual_project);
        }

        projects.push(Project {
            id: -1,
            name: String::from(constants::CREATE_NEW_PROJECT_STRING),
        });

        Ok(projects)
    }

    pub fn get_project_id_from_name(storage: &mut Connection, name: &String) -> Option<i64> {
        let projects = Project::get_projects(storage).unwrap();
        let project = projects.iter().find(|project| {
            println!("Is {} equal to {}?", project.name, *name);
            project.name == *name
            // println!("{:?}", project);
        });
        // TODO: find a way to better work with Option<_> type
        match project {
            Some(p) => Some(p.id),
            None => None,
        }
    }

    pub fn save(&mut self, storage: &mut Connection) {
        Self::initialize_storage(storage);
        storage
            .execute(
                "INSERT INTO project (name) VALUES (?1)
        ",
                (&self.name,),
            )
            .expect("RUSTCLOCK0005: There was a problem when saving a project.");
        self.id = storage.last_insert_rowid();
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
