//! Module in charge of the execution in the right order of the game steps.
//!
//! An instance [`StateMachine`] is created and works in its own thread, thus allowing to have possibly several parts at the same time if
//! one wishes to make the program evolve in that direction.
//!
//! # Examples
//!
//! ```rust
//! mod state_machine;
//!
//! let game_state_machine = state_machine::StateMachine::new_and_start();
//! game_state_machine.start_game();
//! game_state_machine.wait_end_game();
//! ```
//!
//! # Resources
//! The state machine has been realized with the help of :
//! - [Ana Hoverbear](https://hoverbear.org/blog/rust-state-machine-pattern/)
//! - [synul](https://gist.github.com/synul/d9ba086bf75afb3250fc102da7aab569)
//!
//! # Author
//! Pierre-Louis GAUTIER

use crate::{common, game, screen, DEBUG, INFO, TRACE, WARNING};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Public
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// The structure saving all the elements necessary for the good functioning of the state machine
pub struct StateMachine {
    /// The [Sender] to the [Receiver] of the state machine
    sender: Sender<MqMsg>,
    /// The handler of the thread running the state machine
    handler: thread::JoinHandle<()>,
}

impl StateMachine {
    /// Create and launch the state machine, but not the game, see [`StateMachine::start_game`] to finish the game, see [`run`] to see the routine of the thread.
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

    /// Send the signal to the state machine to start the game
    pub fn start_game(&self) {
        INFO!("[StateMachine] Event : Start the game");

        self.sender
            .send(MqMsg {
                event: Event::PlayerOneTurn,
            })
            .expect("[StateMachine] - Fail to start the game");
    }

    /// The signal to the state machine to stop the game and finish its thread
    #[allow(dead_code)]
    pub fn finish_game(&self) {
        INFO!("[StateMachine] Event : Finish the game");

        self.sender
            .send(MqMsg { event: Event::Quit })
            .expect("[StateMachine] - Fail to Finish the game");
    }

    /// Block the current thread while the the thread of the state machine is not finished, see [`StateMachine::finish_game`]
    pub fn wait_end_game(self) {
        self.handler
            .join()
            .expect("[StateMachine] Error when joining the thread");

        INFO!("[StateMachine] Game is finish");
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
    Quit,
}

#[derive(Copy, Clone, PartialEq)]
enum GameWrapper {
    PlayerOneTurn(Game<PlayerOneTurn>),
    PlayerTwoTurn(Game<PlayerTwoTurn>),
    TestGameStatus(Game<TestGameStatus>),
    TestPlayerTurn(Game<TestPlayerTurn>),
    Quit(Game<Quit>),
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct PlayerOneTurn {}

#[derive(Debug, Copy, Clone, PartialEq)]
struct PlayerTwoTurn {}

#[derive(Debug, Copy, Clone, PartialEq)]
struct TestGameStatus {}

#[derive(Debug, Copy, Clone, PartialEq)]
struct TestPlayerTurn {}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Quit {}

#[derive(Debug, Copy, Clone, PartialEq)]
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

fn action_none(_p_sender: &Sender<MqMsg>, _p_screen: &screen::Screen, _p_game: &mut game::Game) {
    INFO!("[StateMachine] - Action : None");
    // Nothing to do
}

fn action_quit(_p_sender: &Sender<MqMsg>, _p_screen: &screen::Screen, _p_game: &mut game::Game) {
    INFO!("[StateMachine] - Action : Quit");
    // Nothing to do
}

fn action_next_turn(
    _p_sender: &Sender<MqMsg>,
    _p_screen: &screen::Screen,
    _p_game: &mut game::Game,
) {
    INFO!("[StateMachine] - Action : Next Turn");
    _p_game.toggle_player();
    _p_screen.send_msg("Next Turn");
    _p_screen.send_game(_p_game);

    match _p_game.current_player() {
        common::Player::PlayerOne => {
            _p_sender
                .send(MqMsg {
                    event: Event::PlayerOneTurn,
                })
                .expect("[StateMachine] - Error : Error occur when sending Event::PlayerOneTurn");
        }
        common::Player::PlayerTwo => {
            _p_sender
                .send(MqMsg {
                    event: Event::PlayerTwoTurn,
                })
                .expect("[StateMachine] - Error : Error occur when sending Event::PlayerTwoTurn");
        }
    }
}

fn action_end_turn(
    _p_sender: &Sender<MqMsg>,
    _p_screen: &screen::Screen,
    _p_game: &mut game::Game,
) {
    INFO!("[StateMachine] - Action : End Turn");
    if _p_game.is_over() {
        _p_screen.send_game(_p_game);

        let winner = _p_game.current_player();
        if winner == common::Player::PlayerOne {
            _p_screen.send_msg("Player one WIN !");
        } else {
            _p_screen.send_msg("Player two WIN !");
        }

        _p_sender
            .send(MqMsg {
                event: Event::EndGame,
            })
            .expect("[StateMachine] Error can not send the event EndGame");
    } else {
        _p_sender
            .send(MqMsg {
                event: Event::NextTurn,
            })
            .expect("[StateMachine] Error can not send the event NextTurn");
    }
}

fn action_player_one(
    _p_sender: &Sender<MqMsg>,
    _p_screen: &screen::Screen,
    _p_game: &mut game::Game,
) {
    INFO!("[StateMachine] - Action : Player one is playing");

    if game::player_turn(_p_screen, _p_game) {
        _p_sender
            .send(MqMsg {
                event: Event::EndTurn,
            })
            .expect("[StateMachine] Error can not send event endTurn");
    } else {
        _p_sender
            .send(MqMsg { event: Event::Quit })
            .expect("[StateMachine] Error can not send event endTurn");
    }
}

fn action_player_two(
    _p_sender: &Sender<MqMsg>,
    _p_screen: &screen::Screen,
    _p_game: &mut game::Game,
) {
    INFO!("[StateMachine] - Action : Player two is playing");

    if game::player_turn(_p_screen, _p_game) {
        _p_sender
            .send(MqMsg {
                event: Event::EndTurn,
            })
            .expect("[StateMachine] Error can not send event endTurn");
    } else {
        _p_sender
            .send(MqMsg { event: Event::Quit })
            .expect("[StateMachine] Error can not send event endTurn");
    }
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

    pub fn is_quit(&self) -> bool {
        match self {
            GameWrapper::Quit(_) => true,
            _ => false,
        }
    }

    pub fn step(
        &self,
        event: &Event,
    ) -> Result<(Self, fn(&Sender<MqMsg>, &screen::Screen, &mut game::Game)), ()> {
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
                Ok((GameWrapper::quit(), action_quit))
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
            (_, Event::Quit) => Ok((GameWrapper::quit(), action_quit)),
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
    let l_screen = screen::Screen::new_and_start();
    let mut l_game: game::Game = game::create_game(&l_screen);

    l_screen.send_game(&l_game);
    while l_current_state.is_quit() != true {
        let l_msg: MqMsg = p_receiver
            .recv()
            .expect("[StateMachine] Error when receiving the message in the channel");

        l_current_state = match l_current_state.step(&l_msg.event) {
            Ok((l_new_state, l_callback)) => {
                l_callback(p_sender, &l_screen, &mut l_game);
                l_new_state
            }
            Err(_) => {
                break; // break loop
            }
        };
    }
    l_screen.stop_and_free();
}
