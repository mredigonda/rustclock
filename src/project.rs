use crate::constants;

pub struct Project {
    pub id: i32,
    pub name: String,
}

impl Project {
    // pub fn new(name: String) -> Self {
    //     Project { id: -1, name }
    // }

    pub fn get_projects() -> Vec<Project> {
        vec![Project {
            id: -1,
            name: String::from(constants::CREATE_NEW_PROJECT_STRING),
        }]
    }
}
