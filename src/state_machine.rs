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
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Public
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Destroy the given `p_state_machine`
pub fn free(p_state_machine: &StateMachine) {
    info!("[StateMachine] Event : Destroy the state machine");
}

/// Ask for establish the connection
pub fn ask_for_connection(p_state_machine: &StateMachine) {
    info!("[StateMachine] Event : Ask for connection");

    p_state_machine.sender.send(MqMsg {
        event: Event::AskForConnection
    });
}

/// Ask for the player to select his role (host or guest)
pub fn ask_for_select_role(p_state_machine: &StateMachine) {
    info!("[StateMachine] Event : Ask for select role");

    p_state_machine.sender.send(MqMsg {
        event: Event::AskForSelectRole
    });
}

/// Signal to the given `p_state_machine` that the connection between the guest and the host is established
pub fn signal_connection_established(p_state_machine: &StateMachine) {
    info!("[StateMachine] Event : Signal the connection is established");

    p_state_machine.sender.send(MqMsg {
        event: Event::SignalConnectionReady
    });
}

/// Signal to the given `p_state_machine` that the game is not finish
pub fn signal_to_continue_the_game(p_state_machine: &StateMachine) {
    info!("[StateMachine] Event : Signal to continue the game");

    p_state_machine.sender.send(MqMsg {
        event: Event::SignalToContinueTheGame
    });
}

/// Signal to the given `p_state_machine` that it is the player's turn to play
pub fn signal_to_play(p_state_machine: &StateMachine) {
    info!("[StateMachine] Event : Signal at the user to play");

    p_state_machine.sender.send(MqMsg {
        event: Event::PlayerTurn
    });
}

/// Signal to the given `p_state_machine` that it is the opponent turn to play
pub fn ask_for_wait_opponent(p_state_machine: &StateMachine) {
    info!("[StateMachine] Event : Ask to wait the opponent");

    p_state_machine.sender.send(MqMsg {
        event: Event::OpponentTurn
    });
}

/// Signal to the given `p_state_machine` that the round is over
pub fn signal_finish_turn(p_state_machine: &StateMachine) {
    info!("[StateMachine] Event : Signal the end of the turn");

    p_state_machine.sender.send(MqMsg {
        event: Event::TurnFinish
    });
}

/// Signal to the given `p_state_machine` that the game is over
pub fn signal_game_finish(p_state_machine: &StateMachine) {
    info!("[StateMachine] Event : Signal the end of the game");

    p_state_machine.sender.send(MqMsg {
        event: Event::GameFinish
    });
}

/// Signal to the given `p_state_machine` that there is a connection error
pub fn signal_error_connection(p_state_machine: &StateMachine) {
    info!("[StateMachine] Event : A connection error occur");

    p_state_machine.sender.send(MqMsg {
        event: Event::ErrorConnection
    });
}

pub struct StateMachine {
    sender: Sender<MqMsg>,
    handler: thread::JoinHandle<()>
}

impl StateMachine {
    pub fn create_and_start() -> Self {
        info!("[StateMachine] Event : Create the state machine");

        let (l_sender, l_receiver): (Sender<MqMsg>, Receiver<MqMsg>) = mpsc::channel();

        Self {
            sender: l_sender,
            handler: thread::spawn(move || {
                run(&l_receiver);
            }),
        }
    }

    pub fn stop_and_destroy(self) {
        info!("[StateMachine] Event : Stop the state machine");

        self.sender.send(MqMsg {
            event: Event::Stop
        });
        self.handler.join().expect("[StateMachine] Error when joining the thread");

        // TODO Destroy
    }
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
    Stop,
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
        // display_role_selection_screen();
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
        Game {
            state: ChoiceForGameStatus {},
        }
    }
}

impl From<&mut Game<WaitingForOpponent>> for Game<ChoiceForGameStatus> {
    fn from(_previous_state: &mut Game<WaitingForOpponent>) -> Game<ChoiceForGameStatus> {
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

fn action_is_my_turn(p_grid: &mut game::Grid) {
    info!("[StateMachine] Action : Test if it's my turn");

    screen::write_in_grid(p_grid, &String::from("X"));
}

fn action_next_turn(p_grid: &game::Grid) {
    info!("[StateMachine] Action : Pass to the next turn");

    screen::display_grid(p_grid);
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

    pub fn step(&mut self, event: &Event, p_grid: &mut game::Grid) -> Result<Self, ()> {
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
                action_is_my_turn(p_grid);
                Ok(GameWrapper::ChoiceForPlayer(previous_state.into()))
            }
            (GameWrapper::ChoiceForGameStatus(previous_state), Event::GameFinish) => {
                Ok(GameWrapper::WaitingForConnection(previous_state.into()))
            }
            (GameWrapper::ChoiceForPlayer(previous_state), Event::PlayerTurn) => {
                action_next_turn(&p_grid);
                Ok(GameWrapper::Playing(previous_state.into()))
            }
            (GameWrapper::ChoiceForPlayer(previous_state), Event::OpponentTurn) => {
                Ok(GameWrapper::WaitingForOpponent(previous_state.into()))
            }
            (GameWrapper::Playing(previous_state), Event::TurnFinish) => {
                Ok(GameWrapper::ChoiceForGameStatus(previous_state.into()))
            }
            (GameWrapper::WaitingForOpponent(previous_state), Event::TurnFinish) => {
                action_next_turn(&p_grid);
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

fn run(p_recv: &Receiver<MqMsg>) {
    info!("[StateMachine] Start the state machine");

    let mut l_current_state: GameWrapper = GameWrapper::new();
    let mut l_grid: game::Grid = game::Grid::new();
    loop{
        let l_msg: MqMsg = p_recv.recv().expect("[StateMachine] Error when receiving the message in the channel");

        match l_msg.event {
            _ => l_current_state = l_current_state.step(&l_msg.event, &mut l_grid).unwrap(),
            Event::Stop => break,
        };


        // TODO trait the msg

        // Use only enum ? (for the state)
        // What data to share ?
    }
}
