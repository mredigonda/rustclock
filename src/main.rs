use inquire::{Confirm, Select, Text};
use rusqlite::Connection;
use std::error;

mod activity;
mod constants;
mod project;
mod time;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut conn = Connection::open("./.rustclock.db3").unwrap();

    let current_activity = activity::Activity::get_current(&mut conn)?;
    if current_activity.is_none() {
        println!("There is no current activity.");
        let create_new = Confirm::new("Do you wish to create one?").prompt()?;
        if create_new {
            let activity_name = Text::new("Activity name:").prompt()?;
            let projects = project::Project::get_projects(&mut conn).unwrap();
            let project_names = projects
                .iter()
                .map(|project| {
                    let x = &project;
                    x.name.clone()
                })
                .collect();
            let mut activity_project = Select::new("Project:", project_names).prompt()?;
            if activity_project == String::from(constants::CREATE_NEW_PROJECT_STRING) {
                let project_name = Text::new("Project name:").prompt()?;
                let mut new_project = project::Project::new(project_name);
                new_project.save(&mut conn);
                activity_project = new_project.name;
            }
            let project_id =
                project::Project::get_project_id_from_name(&mut conn, &activity_project)
                    .expect("RUSTCLOCK0006: Could not get id from project name.");
            let mut new_activity = activity::Activity::new(activity_name, project_id);
            new_activity.save(&mut conn);
            println!("⌛️ Tracking new activity: {}", new_activity.description);
        } else {
            println!("😠 Ok bye.");
        }
    } else {
        let mut activity = current_activity.unwrap();
        println!("{}", activity.get_report(&mut conn));
        let end_now = Confirm::new("Do you wish to end it now?").prompt()?;
        if end_now {
            activity.end_now();
            activity.save(&mut conn);
            println!("Activity was finished 🎉.");
        } else {
            println!("😒 Whatever.");
        }
    }

    // let activity_name = Text::new("Activity name:").prompt()?;
    // let activity_project = Select::new("Project:", get_projects()).prompt()?;
    // let new_activity = activity::Activity::new(activity_name);
    // new_activity.save(&mut conn);

    Ok(())
}
