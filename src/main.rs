use inquire::{error::InquireResult, Select, Text};

mod state;

fn main() -> InquireResult<()> {
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

    Ok(())
}

fn get_projects() -> Vec<String> {
    vec![String::from("<Create new>")]
}
