//! Module in charge of displaying the different messages of the game.
//!
//! An instance [`Screen`] is created and is placed in a separate thread allowing to have a display whose execution is
//! independent of the game management. It is thus possible to have several parts to display at the same time.
//! It is then easy to switch from the display in the terminal to a more complex game window.
//!
//! # Example
//!
//! ```rust
//! mod screen;
//! mod game;
//!
//! let screen = screen::Screen::new_and_start();
//!
//! screen.send_msg("HelloWorld");
//!
//! // Use a game::Game
//! let mut l_game: game::Game = game::create_game(&l_screen);
//!
//! // Set the grid
//! // ...
//!
//! screen.send_game(&game);
//!
//! screen.stop_and_free();
//!
//! ```
//!
//! # Authors
//! - Pierre-Louis GAUTIER
//! - Damien FRISSANT

use crate::game;
use crate::{INFO, TRACE};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Public
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// The structure saving all the elements necessary for the good functioning of the screen
pub struct Screen {
    /// The Sender to the Receiver of the screen
    sender: Sender<MqScreen>,
    /// The handler of the thread running the screen
    handler: thread::JoinHandle<()>,
}

impl Screen {
    /// Create and launch the screen, see [`run`]
    pub fn new_and_start() -> Self {
        INFO!("[Screen] Event : Create the Screen");

        let (l_sender, l_receiver): (Sender<MqScreen>, Receiver<MqScreen>) = mpsc::channel();
        Self {
            sender: l_sender,
            handler: thread::spawn(move || {
                run(&l_receiver);
            }),
        }
    }

    /// Send the to quit the screen ([`Screen::send_quit`]), and block the current thread until the [`Screen`]'s thread is not terminated
    pub fn stop_and_free(self) {
        INFO!("[Screen] Event : Stop the state machine");

        self.send_quit();
        self.handler
            .join()
            .expect("[Screen] Error when joining the thread");

        INFO!("[Screen] Event : Destroy the Screen");
    }

    /// The the signal to stop the [`Screen`]'s thread
    pub fn send_quit(&self) {
        self.sender.send(MqScreen::Quit).expect("[Screen] Error can't send Quit message");
    }

    /// Send a message to display to the [`Screen`]
    pub fn send_msg(&self, p_message: &str) {
        self.sender
            .send(MqScreen::Message {
                msg: String::from(p_message),
            })
            .expect("[Screen] Error can't send a message");
    }

    /// Send a [game::Game] to display to the [`Screen`]
    pub fn send_game(&self, p_game: &game::Game) {
        self.sender
            .send(MqScreen::CurrentGrid {
                game: p_game.clone()
            })
            .expect("[Screen] Error can't send a message");
    }
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Private
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

///Enum of the screen
/// The enum allow us the manage the channel without a communication protocol.
/// See the function [`run`]
enum MqScreen {
    Message { msg: String },
    CurrentGrid { game: game::Game },
    Quit,
}

fn run(p_receiver: &Receiver<MqScreen>) {
    println!("\x1B[2J\x1B[1;1H");

    let mut l_current_game: game::Game;
    loop {
        match p_receiver.recv().expect("[Screen] - Error when receiving message") {
            MqScreen::CurrentGrid { game } => {
                l_current_game = game;
                println!("\x1B[2J\x1B[1;1H");
                println!("{}", l_current_game);
            }

            MqScreen::Message { msg } => {
                println!("{}", msg);
            }
            MqScreen::Quit => {
                println!("Good by");
                break;
            }
        }
    }
}
