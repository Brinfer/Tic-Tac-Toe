mod common;
mod game;
mod screen;
mod state_machine;
mod tools;

fn main() {
    println!("\x1B[2J\x1B[1;1H");
    INFO!("[Main] Program Started");

    let game_state_machine = state_machine::StateMachine::new_and_start();
    game_state_machine.start_game();

    INFO!("[Main] Game started");
    
    game_state_machine.wait_end_game();

    INFO!("[Main] Program finished");
}
