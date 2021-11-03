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

use crate::{com, common, game, screen, DEBUG, ERROR, INFO, TRACE, WARNING};
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Public
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Ask for establish the connection
pub fn ask_for_connection(p_state_machine: &StateMachine) {
    INFO!("[StateMachine] Event : Ask for connection");

    p_state_machine.sender.send(MqMsg {
        event: Event::AskForConnection,
    });
}

/// Ask for the player to select his role (host or guest)
pub fn ask_for_select_role(p_state_machine: &StateMachine) {
    INFO!("[StateMachine] Event : Ask for select role");

    p_state_machine.sender.send(MqMsg {
        event: Event::AskForSelectRole,
    });
}

/// Signal to the given `p_state_machine` that the game is not finish
pub fn signal_to_continue_the_game(p_state_machine: &StateMachine) {
    INFO!("[StateMachine] Event : Signal to continue the game");

    p_state_machine.sender.send(MqMsg {
        event: Event::SignalToContinueTheGame,
    });
}

/// Signal to the given `p_state_machine` that it is the player's turn to play
pub fn signal_to_play(p_state_machine: &StateMachine) {
    INFO!("[StateMachine] Event : Signal at the user to play");

    p_state_machine.sender.send(MqMsg {
        event: Event::PlayerTurn,
    });
}

/// Signal to the given `p_state_machine` that it is the opponent turn to play
pub fn ask_for_wait_opponent(p_state_machine: &StateMachine) {
    INFO!("[StateMachine] Event : Ask to wait the opponent");

    p_state_machine.sender.send(MqMsg {
        event: Event::OpponentTurn,
    });
}

/// Signal to the given `p_state_machine` that the round is over
pub fn signal_finish_turn(p_state_machine: &StateMachine) {
    INFO!("[StateMachine] Event : Signal the end of the turn");

    p_state_machine.sender.send(MqMsg {
        event: Event::TurnFinish,
    });
}

/// Signal to the given `p_state_machine` that the game is over
pub fn signal_game_finish(p_state_machine: &StateMachine) {
    INFO!("[StateMachine] Event : Signal the end of the game");

    p_state_machine.sender.send(MqMsg {
        event: Event::GameFinish,
    });
}

/// Signal to the given `p_state_machine` that there is a connection ERROR
pub fn signal_error_connection(p_state_machine: &StateMachine) {
    INFO!("[StateMachine] Event : A connection ERROR occur");

    p_state_machine.sender.send(MqMsg {
        event: Event::ErrorConnection,
    });
}

pub struct StateMachine {
    sender: Sender<MqMsg>,
    is_host: bool,
    handler: thread::JoinHandle<()>,
}

impl StateMachine {
    pub fn create_and_start() -> Self {
        INFO!("[StateMachine] Event : Create the state machine");

        let (l_sender, l_receiver): (Sender<MqMsg>, Receiver<MqMsg>) = mpsc::channel();

        Self {
            sender: l_sender,
            is_host: false, // Temp value
            handler: thread::spawn(move || {
                run(&l_receiver);
            }),
        }
    }

    pub fn stop_and_destroy(self) {
        INFO!("[StateMachine] Event : Stop the state machine");

        self.sender.send(MqMsg { event: Event::Stop });
        self.handler
            .join()
            .expect("[StateMachine] Error when joining the thread");

        INFO!("[StateMachine] Event : Destroy the state machine");
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Private
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// The different events that can affect the state machine
#[derive(Debug, PartialEq)]
enum Event {
    /// Ask to the player to select a role (host or guest), see [ask_for_select_role]
    AskForSelectRole,
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
    /// Signal to the state machine that there is a connection ERROR, see [signal_error_connection]
    ErrorConnection,
    Stop,
}

enum GameWrapper {
    SelectRole(Game<SelectRole>),
    WaitingForConnectionHost(Game<WaitingForConnectionHost>),
    WaitingForConnectionGuest(Game<WaitingForConnectionGuest>),
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
struct SelectRole {}

#[derive(Debug)]
struct WaitingForConnectionHost {}

#[derive(Debug)]
struct WaitingForConnectionGuest {}

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

impl From<&mut Game<SelectRole>> for Game<WaitingForConnectionHost> {
    fn from(_previous_state: &mut Game<SelectRole>) -> Game<WaitingForConnectionHost> {
        Game {
            state: WaitingForConnectionHost {},
        }
    }
}

impl From<&mut Game<SelectRole>> for Game<WaitingForConnectionGuest> {
    fn from(_previous_state: &mut Game<SelectRole>) -> Game<WaitingForConnectionGuest> {
        Game {
            state: WaitingForConnectionGuest {},
        }
    }
}

impl From<&mut Game<WaitingForConnectionGuest>> for Game<ChoiceForGameStatus> {
    fn from(_previous_state: &mut Game<WaitingForConnectionGuest>) -> Game<ChoiceForGameStatus> {
        Game {
            state: ChoiceForGameStatus {},
        }
    }
}

impl From<&mut Game<WaitingForConnectionHost>> for Game<ChoiceForGameStatus> {
    fn from(_previous_state: &mut Game<WaitingForConnectionHost>) -> Game<ChoiceForGameStatus> {
        Game {
            state: ChoiceForGameStatus {},
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

impl From<&mut Game<ChoiceForGameStatus>> for Game<WaitingForConnectionHost> {
    fn from(_previous_state: &mut Game<ChoiceForGameStatus>) -> Game<WaitingForConnectionHost> {
        Game {
            state: WaitingForConnectionHost {},
        }
    }
}

impl From<&mut Game<ChoiceForPlayer>> for Game<WaitingForConnectionHost> {
    fn from(_previous_state: &mut Game<ChoiceForPlayer>) -> Game<WaitingForConnectionHost> {
        Game {
            state: WaitingForConnectionHost {},
        }
    }
}

impl From<&mut Game<ChoiceForPlayer>> for Game<Playing> {
    fn from(_previous_state: &mut Game<ChoiceForPlayer>) -> Game<Playing> {
        Game { state: Playing {} }
    }
}

impl From<&mut Game<ChoiceForPlayer>> for Game<WaitingForOpponent> {
    fn from(_previous_state: &mut Game<ChoiceForPlayer>) -> Game<WaitingForOpponent> {
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

fn action_select_role() -> common::PlayerRole {
    INFO!("[StateMachine] Action : Select the role");

    screen::display_role_selection_screen()
}

fn action_establish_connection_host() -> Result<TcpStream, ()> {
    INFO!("[StateMachine] Action : Establish connection for the host");
    com::server::set_up()
}

fn action_establish_connection_guest() -> Result<TcpStream, ()> {
    INFO!("[StateMachine] Action : Establish connection for the guest");
    // TODO enter custom ip address and custom port
    com::client::set_up()
}

fn action_choice_game_status(p_grid: &game::Grid) {
    INFO!("[StateMachine] Action : Choice the game status");
    // TODO display Grid, choose if the party is finish
    println!("{}", p_grid);
    game::is_over(p_grid);
}

fn action_is_my_turn(p_grid: &mut game::Grid) {
    INFO!("[StateMachine] Action : Test if it's my turn");

    screen::write_in_grid(p_grid, &String::from("X"));
}

fn action_next_turn(p_grid: &game::Grid) {
    INFO!("[StateMachine] Action : Pass to the next turn");

    println!("{}", p_grid);
}

fn action_exit_game() {
    INFO!("[StateMachine] Action : Exit the game");
}

fn action_play() {
    INFO!("[StateMachine] Action : Player turn");
}

fn action_wait() {
    INFO!("[StateMachine] Action : Opponent turn");
}

fn action_error_connection() {
    INFO!("[StateMachine] Action : Error connection");
}

/////////////////////////////////////////// Functions /////////////////////////////////////////////////////////////////

impl Game<SelectRole> {
    pub fn new() -> Self {
        Game {
            state: SelectRole {},
        }
    }
}

impl Game<ErrorConnection> {
    pub fn error_connection() -> Self {
        Game {
            state: ErrorConnection {},
        }
    }
}

impl GameWrapper {
    pub fn new() -> Self {
        GameWrapper::SelectRole(Game::new())
    }

    pub fn step(
        &mut self,
        event: &Event,
        p_grid: &mut game::Grid,
        p_comm: &mut Option<TcpStream>,
    ) -> Result<Self, ()> {
        match (self, event) {
            (GameWrapper::SelectRole(previous_state), Event::AskForSelectRole) => {
                match action_select_role() {
                    common::PlayerRole::GUEST => {
                        DEBUG!("[StateMachine] Player is the guest");
                        return Ok(GameWrapper::WaitingForConnectionGuest(
                            previous_state.into(),
                        ));
                    }
                    common::PlayerRole::HOST => {
                        DEBUG!("[StateMachine] Player is the host");
                        return Ok(GameWrapper::WaitingForConnectionHost(previous_state.into()));
                    }
                    _ => {
                        DEBUG!("[StateMachine] Player quit the game");
                        return Err(());
                    }
                };
            }
            (GameWrapper::WaitingForConnectionGuest(previous_state), Event::AskForConnection) => {
                match action_establish_connection_guest() {
                    Ok(l_stream) => *p_comm = Some(l_stream),
                    Err(_) => {
                        ERROR! {"[StateMachine] Impossible to connect"}
                        return Err(());
                    }
                }
                action_choice_game_status(&p_grid);
                Ok(GameWrapper::ChoiceForGameStatus(previous_state.into()))
            }
            (GameWrapper::WaitingForConnectionHost(previous_state), Event::AskForConnection) => {
                match action_establish_connection_host() {
                    Ok(l_stream) => *p_comm = Some(l_stream),
                    Err(_) => {
                        ERROR! {"[StateMachine] Impossible to connect"}
                        return Err(());
                    }
                }
                action_choice_game_status(&p_grid);
                Ok(GameWrapper::ChoiceForGameStatus(previous_state.into()))
            }
            (GameWrapper::Playing(previous_state), Event::TurnFinish) => {
                Ok(GameWrapper::ChoiceForGameStatus(previous_state.into()))
            }
            (GameWrapper::WaitingForOpponent(previous_state), Event::TurnFinish) => {
                action_next_turn(&p_grid);
                Ok(GameWrapper::ChoiceForGameStatus(previous_state.into()))
            }
            (GameWrapper::ChoiceForGameStatus(previous_state), Event::SignalToContinueTheGame) => {
                action_is_my_turn(p_grid);
                Ok(GameWrapper::ChoiceForPlayer(previous_state.into()))
            }
            (GameWrapper::ChoiceForGameStatus(previous_state), Event::GameFinish) => {
                action_error_connection();
                Ok(GameWrapper::WaitingForConnectionHost(previous_state.into()))
            }
            (GameWrapper::ChoiceForPlayer(previous_state), Event::PlayerTurn) => {
                action_play();
                Ok(GameWrapper::Playing(previous_state.into()))
            }
            (GameWrapper::ChoiceForPlayer(previous_state), Event::OpponentTurn) => {
                action_wait();
                Ok(GameWrapper::WaitingForOpponent(previous_state.into()))
            }

            (_, Event::ErrorConnection) => {
                WARNING!("[StateMachine] Disconnection");
                action_exit_game();
                Ok(GameWrapper::ErrorConnection(Game::error_connection()))
            }
            (_, _) => {
                ERROR!("[StateMachine] Unsupported transition");
                Err(())
            }
        }
    }
}

fn run(p_recv: &Receiver<MqMsg>) {
    INFO!("[StateMachine] Start the state machine");

    let mut l_current_state: GameWrapper = GameWrapper::new();
    let mut l_grid: game::Grid = game::Grid::new();
    let mut l_comm: Option<TcpStream> = None;

    loop {
        let l_msg: MqMsg = p_recv
            .recv()
            .expect("[StateMachine] Error when receiving the message in the channel");

        match l_msg.event {
            Event::Stop => break,
            _ => {
                l_current_state = l_current_state
                    .step(&l_msg.event, &mut l_grid, &mut l_comm)
                    .unwrap()
            }
        };

        // TODO trait the msg

        // Use only enum ? (for the state)
        // What data to share ?
    }
}
