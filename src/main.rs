use inquire::{error::InquireResult, Select, Text};
use rusqlite::Connection;

mod activity;
mod state;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> InquireResult<()> {
    let new_activity = activity::Activity::new(String::from("nueva tarea"));

    // let current_activity =
    //     activity::Activity::get_current().expect("Oops, parece que no hay actividad actual.");
    let initial_state = state::State::new();

    println!(
        "No current activity! [{}]",
        initial_state.current_activity.unwrap()
    );

    let activity_name = Text::new("Activity name:").prompt()?;
    println!("{}", activity_name);

    let activity_project = Select::new("Project:", get_projects()).prompt()?;
    println!("{}", activity_project);

    println!(
        "Starting activity {} from project {} at {}",
        activity_name, activity_project, 1
    );

    let mut conn = Connection::open("./.rustclock.db3").unwrap();
    new_activity.save(&mut conn);

    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
        id    INTEGER PRIMARY KEY,
        name  TEXT NOT NULL,
        data  BLOB
    )",
        (),
    )
    .unwrap();

    let me = Person {
        id: 0,
        name: activity_name,
        data: None,
    };

    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )
    .unwrap();

    Ok(())
}

fn get_projects() -> Vec<String> {
    vec![String::from("<Create new>")]
}
