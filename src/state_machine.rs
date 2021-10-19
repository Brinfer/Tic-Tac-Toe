//! The state machine allowing to make the game work.
//!
//! # Examples
//!
//! ```rust
//!     mod state_machine;
//!
//!     let game_state_machine : state_machine::StateMachine = state_machine::new();
//!
//!     state_machine::ask_for_select_role(&game_state_machine);
//!     state_machine::ask_for_connection(&game_state_machine);
//! ```
//! # Resources
//! The state machine has been realized with the help of :
//! - [Ana Hoverbear](https://hoverbear.org/blog/rust-state-machine-pattern/)
//! - [synul](https://gist.github.com/synul/d9ba086bf75afb3250fc102da7aab569)
//!
//! # Author
//! Pierre-Louis GAUTIER

use crate::{common, error, game, info, screen, warning};
use posixmq::PosixMq;
use std::mem::size_of;
use std::sync::Mutex;
use std::thread;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Public
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Init the state machine
pub fn new() -> StateMachine {
    info!("[StateMachine] Event : Create the state machine");
    return StateMachine {
        current_state: Mutex::new(GameWrapper::new()),
        grid: game::init_grid()
    };
}

/// Destroy the given `p_state_machine`
pub fn free(p_state_machine: &StateMachine) {
    info!("[StateMachine] Event : Destroy the state machine");

    {
        let mut state_machine = p_state_machine.current_state.lock().unwrap();
        *state_machine = GameWrapper::free();
    }
}

/// Ask for establish the connection
pub fn ask_for_connection(p_state_machine: &StateMachine) {
    info!("[StateMachine] Event : Ask for connection");

    {
        let mut state_changer = p_state_machine.current_state.lock().unwrap();
        match (*state_changer).step(Event::AskForConnection) {
            Ok(new_state) => *state_changer = new_state,
            Err(()) => (),
        }
    }
}

/// Ask for the player to select his role (host or guest)
pub fn ask_for_select_role(p_state_machine: &StateMachine) {
    info!("[StateMachine] Event : Ask for select role");

    {
        let mut state_changer = p_state_machine.current_state.lock().unwrap();
        match (*state_changer).step(Event::AskForSelectRole) {
            Ok(new_state) => *state_changer = new_state,
            Err(()) => (),
        }
    }
}

/// Signal to the given `p_state_machine` that the connection between the guest and the host is established
pub fn signal_connection_established(p_state_machine: &StateMachine) {
    info!("[StateMachine] Event : Signal the connection is established");

    {
        let mut state_changer = p_state_machine.current_state.lock().unwrap();
        match (*state_changer).step(Event::SignalConnectionReady) {
            Ok(new_state) => *state_changer = new_state,
            Err(()) => (),
        }
        screen::write_in_grid(&p_state_machine.grid.lock().unwrap())
    }
}

/// Signal to the given `p_state_machine` that the game is not finish
pub fn signal_to_continue_the_game(p_state_machine: &StateMachine) {
    info!("[StateMachine] Event : Signal to continue the game");

    {
        let mut state_changer = p_state_machine.current_state.lock().unwrap();
        match (*state_changer).step(Event::SignalToContinueTheGame) {
            Ok(new_state) => *state_changer = new_state,
            Err(()) => (),
        }
    }
}

/// Signal to the given `p_state_machine` that it is the player's turn to play
pub fn signal_to_play(p_state_machine: &StateMachine) {
    info!("[StateMachine] Event : Signal at the user to play");

    {
        let mut state_machine = p_state_machine.current_state.lock().unwrap();
        match (*state_machine).step(Event::PlayerTurn) {
            Ok(new_state) => *state_machine = new_state,
            Err(()) => (),
        }
    }
}

/// Signal to the given `p_state_machine` that it is the opponent turn to play
pub fn ask_for_wait_opponent(p_state_machine: &StateMachine) {
    info!("[StateMachine] Event : Ask to wait the opponent");

    {
        let mut state_machine = p_state_machine.current_state.lock().unwrap();
        match (*state_machine).step(Event::OpponentTurn) {
            Ok(new_state) => *state_machine = new_state,
            Err(()) => (),
        }
    }
}

/// Signal to the given `p_state_machine` that the round is over
pub fn signal_finish_turn(p_state_machine: &StateMachine) {
    info!("[StateMachine] Event : Signal the end of the turn");

    {
        let mut state_machine = p_state_machine.current_state.lock().unwrap();
        match (*state_machine).step(Event::TurnFinish) {
            Ok(new_state) => *state_machine = new_state,
            Err(()) => (),
        }
    }
}

/// Signal to the given `p_state_machine` that the game is over
pub fn signal_game_finish(p_state_machine: &StateMachine) {
    info!("[StateMachine] Event : Signal the end of the game");

    {
        let mut state_machine = p_state_machine.current_state.lock().unwrap();
        match (*state_machine).step(Event::GameFinish) {
            Ok(new_state) => *state_machine = new_state,
            Err(()) => (),
        }
    }
}

/// Signal to the given `p_state_machine` that there is a connection error
pub fn signal_error_connection(p_state_machine: &StateMachine) {
    info!("[StateMachine] Event : A connection error occur");

    {
        let mut state_machine = p_state_machine.current_state.lock().unwrap();
        match (*state_machine).step(Event::ErrorConnection) {
            Ok(new_state) => *state_machine = new_state,
            Err(()) => (),
        }
    }
}

pub struct StateMachine {
    current_state: Mutex<GameWrapper>,
    grid : Vec<Vec<String>>,
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Private
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// The different events that can affect the state machine
#[derive(Debug)]
enum Event {
    /// Ask to the player to select a role (host or guest), see [ask_for_select_role]
    AskForSelectRole,
    /// Signal to the state machine that the connection is ready see [signal_connection_established]
    SignalConnectionReady,
    /// Ask the state machine to establish a connection, see [ask_for_connection]
    AskForConnection,
    /// Signal to the state machine that the is not finish, see [signal_to_continue_the_game]
    SignalToContinueTheGame,
    /// Signal to the player that it his turn to play, see [signal_to_play]
    PlayerTurn,
    /// Signal to the player that it is the opponent turn to play, see [ask_for_wait_opponent]
    OpponentTurn,
    /// Signal to the state machine that the round is over see [signal_finish_turn]
    TurnFinish,
    /// Signal to the state machine that the game is over, see [signal_game_finish]
    GameFinish,
    /// Signal to the state machine that there is a connection error, see [signal_error_connection]
    ErrorConnection,
}

enum GameWrapper {
    None,
    Init(Game<Init>),
    SelectRole(Game<SelectRole>),
    WaitingForConnection(Game<WaitingForConnection>),
    ChoiceForGameStatus(Game<ChoiceForGameStatus>),
    ChoiceForPlayer(Game<ChoiceForPlayer>),
    Playing(Game<Playing>),
    WaitingForOpponent(Game<WaitingForOpponent>),
    ErrorConnection(Game<ErrorConnection>),
}

#[derive(Debug)]
struct MqMsg {
    event: Event,
}

#[derive(Debug)]
struct Init {}

#[derive(Debug)]
struct SelectRole {}

#[derive(Debug)]
struct WaitingForConnection {}

#[derive(Debug)]
struct ChoiceForGameStatus {}

#[derive(Debug)]
struct ChoiceForPlayer {}

#[derive(Debug)]
struct WaitingForOpponent {}

#[derive(Debug)]
struct Playing {}

#[derive(Debug)]
struct ErrorConnection {}

#[derive(Debug)]
struct Game<State> {
    state: State,
}

////////////////////////////////////////// Transitions ////////////////////////////////////////////////////////////////

impl From<&mut Game<Init>> for Game<SelectRole> {
    fn from(_previous_state: &mut Game<Init>) -> Game<SelectRole> {
        display_role_selection_screen();
        Game {
            state: SelectRole {},
        }
    }
}

impl From<&mut Game<SelectRole>> for Game<WaitingForConnection> {
    fn from(_previous_state: &mut Game<SelectRole>) -> Game<WaitingForConnection> {
        action_display_connection_screen();
        Game {
            state: WaitingForConnection {},
        }
    }
}

impl From<&mut Game<ChoiceForGameStatus>> for Game<ChoiceForPlayer> {
    fn from(_previous_state: &mut Game<ChoiceForGameStatus>) -> Game<ChoiceForPlayer> {
        action_is_my_turn();
        Game {
            state: ChoiceForPlayer {},
        }
    }
}

impl From<&mut Game<ChoiceForGameStatus>> for Game<WaitingForConnection> {
    fn from(_previous_state: &mut Game<ChoiceForGameStatus>) -> Game<WaitingForConnection> {
        action_exit_game();
        Game {
            state: WaitingForConnection {},
        }
    }
}

impl From<&mut Game<ChoiceForPlayer>> for Game<WaitingForConnection> {
    fn from(_previous_state: &mut Game<ChoiceForPlayer>) -> Game<WaitingForConnection> {
        action_error_connection();
        Game {
            state: WaitingForConnection {},
        }
    }
}

impl From<&mut Game<WaitingForConnection>> for Game<ChoiceForGameStatus> {
    fn from(_previous_state: &mut Game<WaitingForConnection>) -> Game<ChoiceForGameStatus> {
        action_establish_connection();
        Game {
            state: ChoiceForGameStatus {},
        }
    }
}

impl From<&mut Game<ChoiceForPlayer>> for Game<Playing> {
    fn from(_previous_state: &mut Game<ChoiceForPlayer>) -> Game<Playing> {
        action_play();
        Game { state: Playing {} }
    }
}

impl From<&mut Game<ChoiceForPlayer>> for Game<WaitingForOpponent> {
    fn from(_previous_state: &mut Game<ChoiceForPlayer>) -> Game<WaitingForOpponent> {
        action_wait();
        Game {
            state: WaitingForOpponent {},
        }
    }
}

impl From<&mut Game<Playing>> for Game<ChoiceForGameStatus> {
    fn from(_previous_state: &mut Game<Playing>) -> Game<ChoiceForGameStatus> {
        action_next_turn();
        Game {
            state: ChoiceForGameStatus {},
        }
    }
}

impl From<&mut Game<WaitingForOpponent>> for Game<ChoiceForGameStatus> {
    fn from(_previous_state: &mut Game<WaitingForOpponent>) -> Game<ChoiceForGameStatus> {
        action_next_turn();
        Game {
            state: ChoiceForGameStatus {},
        }
    }
}

//////////////////////////////////////////// Actions //////////////////////////////////////////////////////////////////

fn display_role_selection_screen() {
    info!("[StateMachine] Action : Display the selection screen");

    let user_role = screen::role_selection();

    match user_role {
        common::PlayerRole::HOST => {
            // com::server::main_server();
        }
        common::PlayerRole::GUEST => {
            // com::client::main();
        }
        _ => {
            warning!("Unknown role");
            panic!("Unknown role");
        }
    }
}

fn action_establish_connection() {
    info!("[StateMachine] Action : Start the game");
    // game::init_game();
}

fn action_display_connection_screen() {
    info!("[StateMachine] Action : Display the connection screen");
}

fn action_is_my_turn() {
    info!("[StateMachine] Action : Test if it's my turn");
}

fn action_next_turn() {
    info!("[StateMachine] Action : Pass to the next turn");
}

fn action_exit_game() {
    info!("[StateMachine] Action : Exit the game");
}

fn action_play() {
    info!("[StateMachine] Action : Player turn");
}

fn action_wait() {
    info!("[StateMachine] Action : Opponent turn");
}

fn action_error_connection() {
    info!("[StateMachine] Action : Error connection");
}

/////////////////////////////////////////// Functions /////////////////////////////////////////////////////////////////

impl Game<Init> {
    pub fn new() -> Self {
        Game { state: Init {} }
    }
}

impl GameWrapper {
    pub fn new() -> Self {
        GameWrapper::Init(Game::new())
    }

    pub fn free() -> Self {
        GameWrapper::None
    }

    pub fn step(&mut self, event: Event) -> Result<Self, ()> {
        match (self, event) {
            (GameWrapper::Init(previous_state), Event::AskForSelectRole) => {
                Ok(GameWrapper::SelectRole(previous_state.into()))
            }
            (GameWrapper::SelectRole(previous_state), Event::AskForConnection) => {
                Ok(GameWrapper::WaitingForConnection(previous_state.into()))
            }
            (GameWrapper::WaitingForConnection(previous_state), Event::SignalConnectionReady) => {
                Ok(GameWrapper::ChoiceForGameStatus(previous_state.into()))
            }
            (GameWrapper::ChoiceForGameStatus(previous_state), Event::SignalToContinueTheGame) => {
                Ok(GameWrapper::ChoiceForPlayer(previous_state.into()))
            }
            (GameWrapper::ChoiceForGameStatus(previous_state), Event::GameFinish) => {
                Ok(GameWrapper::WaitingForConnection(previous_state.into()))
            }
            (GameWrapper::ChoiceForPlayer(previous_state), Event::PlayerTurn) => {
                Ok(GameWrapper::Playing(previous_state.into()))
            }
            (GameWrapper::ChoiceForPlayer(previous_state), Event::OpponentTurn) => {
                Ok(GameWrapper::WaitingForOpponent(previous_state.into()))
            }
            (GameWrapper::Playing(previous_state), Event::TurnFinish) => {
                Ok(GameWrapper::ChoiceForGameStatus(previous_state.into()))
            }
            (GameWrapper::WaitingForOpponent(previous_state), Event::TurnFinish) => {
                Ok(GameWrapper::ChoiceForGameStatus(previous_state.into()))
            }
            (_, Event::ErrorConnection) => {
                warning!("[StateMachine] Disconnection");
                Err(())
            }
            (_, _) => {
                error!("[StateMachine] Unsupported transition");
                Err(())
            }
        }
    }
}

fn set_up_mq(p_mq_name: &str) -> PosixMq {
    return posixmq::OpenOptions::readwrite()
        .create()
        .max_msg_len(size_of::<[MqMsg; 1]>())
        .capacity(10)
        .open(p_mq_name)
        .expect("[StateMachine] Error when opening the mqueue");
}

fn tear_down_mq(p_mq_name: &str) {
    posixmq::remove_queue(p_mq_name).unwrap();
}
