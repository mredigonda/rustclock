use rusqlite::Connection;
use std::error;

mod activity;
mod time;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut conn = Connection::open("./.rustclock.db3").unwrap();

    let current_activity = activity::Activity::get_current(&mut conn)?;
    if current_activity.is_none() {
        println!("There is no current activity.");
    } else {
        let mut activity = current_activity.unwrap();
        activity.end_now();
        activity.save(&mut conn);
        println!("Current activity: {:?}", activity);
    }

    // let activity_name = Text::new("Activity name:").prompt()?;
    // let activity_project = Select::new("Project:", get_projects()).prompt()?;
    // let new_activity = activity::Activity::new(activity_name);
    // new_activity.save(&mut conn);

    Ok(())
}

// fn get_projects() -> Vec<String> {
//     vec![String::from("<Create new>")]
// }
