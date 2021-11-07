//!
//! # Author
//! Pierre-Louis GAUTIER
//! Damien FRISSANT

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

///Enum of the screen
/// The enum allow us the manage the channel without a communication protocol
/// See the function "run"
enum MqScreen {
    Message { msg: String },
    CurrentGrid { grid: game::Grid },
    Quit,
}

pub struct Screen {
    sender: Sender<MqScreen>,
    handler: thread::JoinHandle<()>,
}

impl Screen {
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

    pub fn stop_and_free(self) {
        INFO!("[Screen] Event : Stop the state machine");

        self.sender.send(MqScreen::Quit).expect("[Screen] Error can't send message Quit");
        self.handler
            .join()
            .expect("[Screen] Error when joining the thread");

        INFO!("[Screen] Event : Destroy the Screen");
    }

    pub fn send_quit(&self) {
        self.sender.send(MqScreen::Quit).expect("[Screen] Error can't send Quit message");
    }

    pub fn send_msg(&self, p_message: &str) {
        self.sender
            .send(MqScreen::Message {
                msg: String::from(p_message),
            })
            .expect("[Screen] Error can't send a message");
    }

    pub fn send_grid(&self, p_grid: game::Grid) {
        self.sender
            .send(MqScreen::CurrentGrid {
                grid: p_grid
            })
            .expect("[Screen] Error can't send a message");
    }
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Private
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn run(p_receiver: &Receiver<MqScreen>) {
    println!("\x1B[2J\x1B[1;1H");

    let mut l_current_grid: game::Grid;
    loop {
        match p_receiver.recv().expect("[Screen] - Error when receiving message") {
            MqScreen::CurrentGrid { grid } => {
                l_current_grid = grid;
                println!("\x1B[2J\x1B[1;1H");
                println!("{}", l_current_grid);
            }

            MqScreen::Message { msg } => {
                println!("{}", msg);
            }
            MqScreen::Quit => {
                println!("Good by");
                std::process::exit(0);
            }
        }
    }
}
