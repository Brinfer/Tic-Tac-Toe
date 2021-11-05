mod common;
mod game;
mod screen;
mod state_machine;
mod tools;

fn main() {
    INFO!("[Main] Program Started");

    let game_state_machine = state_machine::StateMachine::new_and_start();

    game_state_machine.player_one_turn();
    game_state_machine.player_two_turn();
    game_state_machine.stop_and_free();

    // state_machine::signal_to_play(&game_state_machine);
    // state_machine::signal_finish_turn(&game_state_machine);
    // state_machine::signal_to_continue_the_game(&game_state_machine);
    // state_machine::ask_for_wait_opponent(&game_state_machine);
    // state_machine::signal_finish_turn(&game_state_machine);
    // state_machine::signal_game_finish(&game_state_machine);

    // game_state_machine.stop_and_destroy();

    INFO!("[Main] Program finished");
}
