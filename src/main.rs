use inquire::{Confirm, Select, Text};
use rusqlite::Connection;
use std::error;

mod activity;
mod time;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut conn = Connection::open("./.rustclock.db3").unwrap();

    let current_activity = activity::Activity::get_current(&mut conn)?;
    if current_activity.is_none() {
        println!("There is no current activity.");
        let create_new = Confirm::new("Do you wish to create one?").prompt()?;
        if create_new {
            let activity_name = Text::new("Activity name:").prompt()?;
            let activity_project = Select::new("Project:", get_projects()).prompt()?;
            let new_activity = activity::Activity::new(activity_name);
            new_activity.save(&mut conn);
        } else {
            println!("😠 Ok bye.");
        }
    } else {
        let mut activity = current_activity.unwrap();
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

fn get_projects() -> Vec<String> {
    vec![String::from("<Create new>")]
}
