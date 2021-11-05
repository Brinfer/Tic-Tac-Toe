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

use crate::{game, screen, DEBUG, INFO, TRACE, WARNING};
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

        self.sender
            .send(MqMsg { event: Event::Quit })
            .expect("Can not send event Quit");
        self.handler
            .join()
            .expect("[StateMachine] Error when joining the thread");

        INFO!("[StateMachine] Event : Destroy the state machine");
    }

    ///////////////////////////////////////////////////// Events //////////////////////////////////////////////////////

    pub fn player_one_turn(&self) {
        INFO!("[StateMachine] - Event : Signal player one turn");

        self.sender
            .send(MqMsg {
                event: Event::PlayerOneTurn,
            })
            .expect("Can not send the event PlayerOneTurn");
    }

    pub fn player_two_turn(&self) {
        INFO!("[StateMachine] - Event : Signal player two turn");

        self.sender
            .send(MqMsg {
                event: Event::PlayerTwoTurn,
            })
            .expect("Can not send the event PlayerTwoTurn");
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
    //Error,
    Quit,
}

#[derive(Copy, Clone)]
enum GameWrapper {
    PlayerOneTurn(Game<PlayerOneTurn>),
    PlayerTwoTurn(Game<PlayerTwoTurn>),
    TestGameStatus(Game<TestGameStatus>),
    TestPlayerTurn(Game<TestPlayerTurn>),
    //Quit(Game<Quit>),
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

fn action_none(_p_sender: &Sender<MqMsg>, _p_screen: &screen::Screen, _p_grid: &mut game::Grid) {
    INFO!("[StateMachine] - Action : None");
    // Nothing to do
}

fn action_quit(_p_sender: &Sender<MqMsg>, _p_screen: &screen::Screen, _p_grid: &mut game::Grid) {
    INFO!("[StateMachine] - Action : Quit");
    _p_screen.send(screen::MqScreen::Quit);
}

fn action_next_turn(
    _p_sender: &Sender<MqMsg>,
    _p_screen: &screen::Screen,
    _p_grid: &mut game::Grid,
) {
    INFO!("[StateMachine] - Action : Next Turn");
    _p_grid.toggle_player();
    _p_screen.send(screen::MqScreen::Message {
        msg: String::from("Next Turn"),
    });
    _p_screen.send(screen::MqScreen::CurrentGrid {
        grid: _p_grid.clone(),
    })
}

fn action_end_turn(
    _p_sender: &Sender<MqMsg>,
    _p_screen: &screen::Screen,
    _p_grid: &mut game::Grid,
) {
    INFO!("[StateMachine] - Action : End Turn");
    if game::is_over(&_p_grid) {
        _p_sender
            .send(MqMsg {
                event: Event::EndGame,
            })
            .expect("Can not send the event EndGame");
    } else {
        _p_sender
            .send(MqMsg {
                event: Event::NextTurn,
            })
            .expect("Can not send the event NextTurn");
    }
}

fn action_player_one(
    _p_sender: &Sender<MqMsg>,
    _p_screen: &screen::Screen,
    _p_grid: &mut game::Grid,
) {
    INFO!("[StateMachine] - Action : Player one is playing");
    _p_screen.send(screen::MqScreen::Message {
        msg: String::from("Player one it is your turn"),
    });
    game::player_turn(_p_grid);
    _p_sender
        .send(MqMsg {
            event: Event::EndTurn,
        })
        .expect("Can not send event endTurn");
}

fn action_player_two(
    _p_sender: &Sender<MqMsg>,
    _p_screen: &screen::Screen,
    _p_grid: &mut game::Grid,
) {
    INFO!("[StateMachine] - Action : Player two is playing");
    _p_screen.send(screen::MqScreen::Message {
        msg: String::from("Player two it is your turn"),
    });
    game::player_turn(_p_grid);
    _p_sender
        .send(MqMsg {
            event: Event::EndTurn,
        })
        .expect("Can not send event endTurn");
}

/////////////////////////////////////////// Functions /////////////////////////////////////////////////////////////////

impl Game<TestPlayerTurn> {
    pub fn new() -> Self {
        Game {
            state: TestPlayerTurn {},
        }
    }
}

/* impl Game<Quit> {
    pub fn quit() -> Self {
        Game { state: Quit {} }
    }
} */

impl GameWrapper {
    pub fn new() -> Self {
        GameWrapper::TestPlayerTurn(Game::new())
    }

    /*     pub fn quit() -> Self {
        GameWrapper::Quit(Game::quit())
    } */

    pub fn step(
        &self,
        event: &Event,
    ) -> Result<(Self, fn(&Sender<MqMsg>, &screen::Screen, &mut game::Grid)), ()> {
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
    let mut l_grid: game::Grid = game::Grid::new(get_integer());
    let l_screen = screen::Screen::new_and_start();
    loop {
        l_screen.send(screen::MqScreen::CurrentGrid {
            grid: l_grid.clone(),
        });
        let l_msg: MqMsg = p_receiver
            .recv()
            .expect("[StateMachine] Error when receiving the message in the channel");

        l_current_state = match l_current_state.step(&l_msg.event) {
            Ok((l_new_state, l_callback)) => {
                l_callback(p_sender, &l_screen, &mut l_grid);
                l_new_state
            }
            Err(_) => {
                break; // break loop
            }
        };
    }
    l_screen.stop_and_free();
}

fn get_integer() -> usize{
    loop {
        println!("Enter the size of the grid you want: ");
        let l_grid_size = &*game::read_keyboard();

        let size_in: u16 = match l_grid_size.trim().parse::<u16>(){
            Ok(as_int) => as_int,
            Err(_) => continue,
        };

        let size_in: usize = size_in as usize;
        return size_in;
    }
}