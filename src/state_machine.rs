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

use crate::{screen, DEBUG, ERROR, INFO, TRACE, WARNING};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Public
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct StateMachine {
    sender: Sender<MqMsg>,
    handler: thread::JoinHandle<()>,
}

impl StateMachine {
    pub fn new_and_start() -> Self {
        INFO!("[StateMachine] Event : Create the state machine");

        let (l_sender, l_receiver): (Sender<MqMsg>, Receiver<MqMsg>) = mpsc::channel();
        let l_sender_copy: Sender<MqMsg> = l_sender.clone();
        Self {
            sender: l_sender,
            handler: thread::spawn(move || {
                run(&l_sender_copy, &l_receiver);
            }),
        }
    }

    pub fn stop_and_free(self) {
        INFO!("[StateMachine] Event : Stop the state machine");

        self.sender.send(MqMsg { event: Event::Quit });
        self.handler
            .join()
            .expect("[StateMachine] Error when joining the thread");

        INFO!("[StateMachine] Event : Destroy the state machine");
    }

    ///////////////////////////////////////////////////// Events //////////////////////////////////////////////////////

    pub fn player_one_turn(&self) {
        INFO!("[StateMachine] - Event : Signal player one turn");

        self.sender.send(MqMsg {
            event: Event::PlayerOneTurn,
        });
    }

    pub fn player_two_turn(&self) {
        INFO!("[StateMachine] - Event : Signal player two turn");

        self.sender.send(MqMsg {
            event: Event::PlayerTwoTurn,
        });
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Private
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct MqMsg {
    event: Event,
}

/////////////////////////////////////////////////////// Events ////////////////////////////////////////////////////////

/// The different events that can affect the state machine
#[derive(Debug, PartialEq)]
enum Event {
    EndTurn,
    NextTurn,
    EndGame,
    PlayerOneTurn,
    PlayerTwoTurn,
    Error,
    Quit,
}

#[derive(Copy, Clone)]
enum GameWrapper {
    PlayerOneTurn(Game<PlayerOneTurn>),
    PlayerTwoTurn(Game<PlayerTwoTurn>),
    TestGameStatus(Game<TestGameStatus>),
    TestPlayerTurn(Game<TestPlayerTurn>),
    Quit(Game<Quit>),
}

#[derive(Debug, Copy, Clone)]
struct PlayerOneTurn {}

#[derive(Debug, Copy, Clone)]
struct PlayerTwoTurn {}

#[derive(Debug, Copy, Clone)]
struct TestGameStatus {}

#[derive(Debug, Copy, Clone)]
struct TestPlayerTurn {}

#[derive(Debug, Copy, Clone)]
struct Quit {}

#[derive(Debug, Copy, Clone)]
struct Game<State> {
    state: State,
}

////////////////////////////////////////// Transitions ////////////////////////////////////////////////////////////////

impl From<&Game<PlayerOneTurn>> for Game<TestGameStatus> {
    fn from(_previous_state: &Game<PlayerOneTurn>) -> Game<TestGameStatus> {
        DEBUG!("[StateMachine] - Transition : From PlayerOneTurn to TestGameStatus");
        Game {
            state: TestGameStatus {},
        }
    }
}

impl From<&Game<PlayerTwoTurn>> for Game<TestGameStatus> {
    fn from(_previous_state: &Game<PlayerTwoTurn>) -> Game<TestGameStatus> {
        DEBUG!("[StateMachine] - Transition : From PlayerTwoTurn to TestGameStatus");
        Game {
            state: TestGameStatus {},
        }
    }
}

impl From<&Game<TestGameStatus>> for Game<TestPlayerTurn> {
    fn from(_previous_state: &Game<TestGameStatus>) -> Game<TestPlayerTurn> {
        DEBUG!("[StateMachine] - Transition : From TestGameStatus to TestPlayerTurn");
        Game {
            state: TestPlayerTurn {},
        }
    }
}

impl From<&Game<TestPlayerTurn>> for Game<PlayerOneTurn> {
    fn from(_previous_state: &Game<TestPlayerTurn>) -> Game<PlayerOneTurn> {
        DEBUG!("[StateMachine] - Transition : From TestPlayerTurn to PlayerOneTurn");
        Game {
            state: PlayerOneTurn {},
        }
    }
}

impl From<&Game<TestPlayerTurn>> for Game<PlayerTwoTurn> {
    fn from(_previous_state: &Game<TestPlayerTurn>) -> Game<PlayerTwoTurn> {
        DEBUG!("[StateMachine] - Transition : From TestPlayerTurn to PlayerTwoTurn");
        Game {
            state: PlayerTwoTurn {},
        }
    }
}

//////////////////////////////////////////// Actions //////////////////////////////////////////////////////////////////

fn action_none(_p_sender: &Sender<MqMsg>, _p_screen: &screen::Screen) {
    INFO!("[StateMachine] - Action : None");
    // Nothing to do
}

fn action_quit(_p_sender: &Sender<MqMsg>, _p_screen: &screen::Screen) {
    INFO!("[StateMachine] - Action : Quit");
   _p_screen.send(screen::MqScreen::Message{msg :String::from("QUIT")});
    // TODO
}

fn action_next_turn(_p_sender: &Sender<MqMsg>, _p_screen: &screen::Screen) {
    INFO!("[StateMachine] - Action : Next Turn");
    // TODO
}

fn action_end_turn(_p_sender: &Sender<MqMsg>, _p_screen: &screen::Screen) {
    INFO!("[StateMachine] - Action : End Turn");
    // TODO Test winner

    // There is a winner
    _p_sender.send(MqMsg {
        event: Event::EndGame,
    });
}

fn action_player_one(_p_sender: &Sender<MqMsg>, _p_screen: &screen::Screen) {
    INFO!("[StateMachine] - Action : Player one is playing");

    // TODO capture keyboard, set gird
}

fn action_player_two(_p_sender: &Sender<MqMsg>, _p_screen: &screen::Screen) {
    INFO!("[StateMachine] - Action : Player two is playing");

    // TODO capture keyboard, set grid
}

/////////////////////////////////////////// Functions /////////////////////////////////////////////////////////////////

impl Game<TestPlayerTurn> {
    pub fn new() -> Self {
        Game {
            state: TestPlayerTurn {},
        }
    }
}

impl Game<Quit> {
    pub fn quit() -> Self {
        Game { state: Quit {} }
    }
}

impl GameWrapper {
    pub fn new() -> Self {
        GameWrapper::TestPlayerTurn(Game::new())
    }

    pub fn quit() -> Self {
        GameWrapper::Quit(Game::quit())
    }

    pub fn step(&self, event: &Event) -> Result<(Self, fn(&Sender<MqMsg>, &screen::Screen)), ()> {
        match (self, event) {
            (GameWrapper::PlayerOneTurn(_previous_state), Event::EndTurn) => Ok((
                GameWrapper::TestGameStatus(_previous_state.into()),
                action_end_turn,
            )),
            (GameWrapper::PlayerTwoTurn(_previous_state), Event::EndTurn) => Ok((
                GameWrapper::TestGameStatus(_previous_state.into()),
                action_end_turn,
            )),
            (GameWrapper::TestGameStatus(_previous_state), Event::EndGame) => {
                Ok((*self, action_quit))
            }
            (GameWrapper::TestGameStatus(_previous_state), Event::NextTurn) => Ok((
                GameWrapper::TestPlayerTurn(_previous_state.into()),
                action_next_turn,
            )),
            (GameWrapper::TestPlayerTurn(_previous_state), Event::PlayerOneTurn) => Ok((
                GameWrapper::PlayerOneTurn(_previous_state.into()),
                action_player_one,
            )),
            (GameWrapper::TestPlayerTurn(_previous_state), Event::PlayerTwoTurn) => Ok((
                GameWrapper::PlayerTwoTurn(_previous_state.into()),
                action_player_two,
            )),
            (_, Event::Quit) => Ok((*self, action_quit)),
            (_, Event::Error) => {
                ERROR!("[StateMachine] Transition : An error occur");
                Err(())
            }
            (_, _) => {
                WARNING!("[StateMachine] - Transition : From ... to ... >> Unsupported transition");
                Ok((*self, action_none))
            }
        }
    }
}

fn run(p_sender: &Sender<MqMsg>, p_receiver: &Receiver<MqMsg>) {
    INFO!("[StateMachine] Start the state machine");

    let mut l_current_state: GameWrapper = GameWrapper::new();
    // let mut l_grid: game::Grid = game::Grid::new();
    let l_screen = screen::Screen::new_and_start();
    loop {
        let l_msg: MqMsg = p_receiver
            .recv()
            .expect("[StateMachine] Error when receiving the message in the channel");

        l_current_state = match l_current_state.step(&l_msg.event) {
            Ok((l_new_state, l_callback)) => {
                l_callback(p_sender, &l_screen);
                l_new_state
            }
            Err(_) => {
                break; // break loop
            }
        };
    }
    l_screen.stop_and_free();
}
