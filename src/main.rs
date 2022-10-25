mod state;

fn main() {
    let initial_state = state::State::new();

    println!("Hello, world! {}", initial_state.current_activity.unwrap());
}
