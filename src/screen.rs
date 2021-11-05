//! TODO
//!
//! # Author
//! Pierre-Louis GAUTIER

use crate::game;
use crate::{DEBUG, ERROR, INFO, TRACE, WARNING};
use std::io::stdin;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Public
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

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

        self.sender.send(MqScreen::Quit).expect("Can not quit");
        self.handler
            .join()
            .expect("[Screen] Error when joining the thread");

        INFO!("[Screen] Event : Destroy the Screen");
    }

    pub fn send(&self, p_message: MqScreen){
        self.sender.send(p_message).expect("Can not send");
    }
}

pub fn write_in_grid(p_grid: &mut game::Grid, p_value: &String) {
    println!("{}", p_grid);
    println!("Enter the number of the box you wish to fill in");
    let mut is_valid: bool = false;
    
}

fn run(p_sender: &Sender<MqScreen>, p_receiver: &Receiver<MqScreen>) {
    let mut l_current_grid: game::Grid;
    loop {
        match p_receiver.recv().expect("Error when receving message") {
            MqScreen::CurrentGrid { grid } => {
                l_current_grid = grid;
                println!("{}", l_current_grid);
            }

            MqScreen::Message { msg } => {
                println!("{}", msg);
            }
            MqScreen::Quit => { 
                println!("QUIT");
                break;}
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Private
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

