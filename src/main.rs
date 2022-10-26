use inquire::{Select, Text};
use rusqlite::Connection;
use std::error;

mod activity;
mod time;

fn main() -> Result<(), Box<dyn error::Error>> {
    let activity_name = Text::new("Activity name:").prompt()?;
    println!("{}", activity_name);

    let activity_project = Select::new("Project:", get_projects()).prompt()?;
    println!("{}", activity_project);

    println!(
        "Starting activity {} from project {} at {}",
        activity_name, activity_project, 1
    );

    let new_activity = activity::Activity::new(activity_name);

    let mut conn = Connection::open("./.rustclock.db3").unwrap();
    new_activity.save(&mut conn);

    let mut current_activity = activity::Activity::get_current(&mut conn)?;
    println!("Current activity: {:?}", current_activity);

    current_activity.end_now();
    current_activity.save(&mut conn);

    Ok(())
}

fn get_projects() -> Vec<String> {
    vec![String::from("<Create new>")]
}
