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
            let projects = project::Project::get_projects();
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
                println!(
                    "TODO: Should try to create new project named: {}",
                    project_name
                );
                activity_project = String::from("new project!");
            } else {
                // todo: also get new project id, to use it in activity...
            }
            println!("Should be created in project: {}", activity_project);
            let new_activity = activity::Activity::new(activity_name);
            new_activity.save(&mut conn);
        } else {
            println!("😠 Ok bye.");
        }
    } else {
        let mut activity = current_activity.unwrap();
        println!("{}", activity.get_report());
        let end_now = Confirm::new("Do you wish to end it now?").prompt()?;
        if end_now {
            activity.end_now();
            activity.save(&mut conn);
            println!("Task was finished 🎉.");
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
