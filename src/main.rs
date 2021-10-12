mod state_machine;
mod game;
mod screen;
mod com;

mod tools;

// mod tooutLesFichiers

fn main() {
    let game_state_machine : state_machine::StateMachine;

    game_state_machine = state_machine::new();

    state_machine::ask_for_select_role(&game_state_machine);
    state_machine::ask_for_connection(&game_state_machine);
    state_machine::signal_connection_established(&game_state_machine);
    state_machine::signal_to_continue_the_game(&game_state_machine);

    state_machine::signal_to_play(&game_state_machine);
    state_machine::signal_finish_turn(&game_state_machine);
    state_machine::signal_to_continue_the_game(&game_state_machine);
    state_machine::ask_for_wait_opponent(&game_state_machine);
    state_machine::signal_finish_turn(&game_state_machine);
    state_machine::signal_game_finish(&game_state_machine);
}
