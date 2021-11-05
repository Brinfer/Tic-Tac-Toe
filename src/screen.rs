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
pub enum MqScreen {
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
        let l_sender_copy: Sender<MqScreen> = l_sender.clone();
        Self {
            sender: l_sender,
            handler: thread::spawn(move || {
                run(&l_sender_copy, &l_receiver);
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

    pub fn send(&self, p_message: MqScreen) {
        self.sender.send(p_message).expect("[Screen] Error can't send a message");
    }

    pub fn send_msg(&self, p_message: &str) {
        self.sender
            .send(MqScreen::Message {
                msg: String::from(p_message),
            })
            .expect("[Screen] Error can't send a message");
    }
}

fn run(p_sender: &Sender<MqScreen>, p_receiver: &Receiver<MqScreen>) {
    let mut l_current_grid: game::Grid;
    loop {
        match p_receiver.recv().expect("[Screen] - Error when receiving message") {
            MqScreen::CurrentGrid { grid } => {
                l_current_grid = grid;
                println!("{}", l_current_grid);
            }

            MqScreen::Message { msg } => {
                println!("{}", msg);
            }
            MqScreen::Quit => {
                println!("QUIT");
                break;
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Private
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
