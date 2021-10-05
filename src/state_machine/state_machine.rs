#[path = "../tools.rs"]
#[macro_use]
mod tools;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Public
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Init and create the state machine
pub fn new() {
    info!("Create the state machine");

    unsafe {
        STATE_MACHINE = GameWrapper::new();
    }
}

/// Destroy the state machine
pub fn free() {
    info!("Destroy the state machine");

    unsafe {
        STATE_MACHINE = GameWrapper::None;
    }
}

pub fn ask_for_connection() {
    info!("Ask for connection");

    unsafe {
        STATE_MACHINE = STATE_MACHINE.step(Event::AskForConnection);
    }
}

pub fn ask_for_select_role() {
    info!("Ask for select role");

    unsafe {
        STATE_MACHINE = STATE_MACHINE.step(Event::AskForSelectRole);
    }
}

pub fn signal_to_continue_the_game() {
    info!("Signal to continue the game");

    unsafe {
        STATE_MACHINE = STATE_MACHINE.step(Event::SignalToContinueTheGame);
    }
}

pub fn signal_to_play() {
    info!("Signal at the user to play");

    unsafe {
        STATE_MACHINE = STATE_MACHINE.step(Event::PlayerTurn);
    }
}

pub fn ask_for_wait_opponent() {
    info!("Ask to wait the opponent");

    unsafe {
        STATE_MACHINE = STATE_MACHINE.step(Event::OpponentTurn);
    }
}

pub fn signal_finish_turn() {
    info!("Signal the end of the turn");

    unsafe {
        STATE_MACHINE = STATE_MACHINE.step(Event::TurnFinish);
    }
}

pub fn signal_game_finish() {
    info!("Signal the end of the game");

    unsafe {
        STATE_MACHINE = STATE_MACHINE.step(Event::GameFinish);
    }
}

pub fn error_connection() {
    info!("A connection error occur");

    unsafe {
        STATE_MACHINE = STATE_MACHINE.step(Event::ErrorConnection);
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Private
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// The current state of the state machine
static mut STATE_MACHINE: GameWrapper = GameWrapper::None;

/// The different events that can affect the state machine
enum Event {
    AskForSelectRole,
    SignalConnectionReady,
    AskForConnection,
    SignalToContinueTheGame,
    PlayerTurn,
    OpponentTurn,
    TurnFinish,
    GameFinish,
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
struct Game<State> {
    state: State,
}

// https://hoverbear.org/blog/rust-state-machine-pattern/
// https://gist.github.com/synul/d9ba086bf75afb3250fc102da7aab569

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
        display_connection_screen();
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
        exit_game();
        Game {
            state: WaitingForConnection {},
        }
    }
}

impl From<&mut Game<ChoiceForPlayer>> for Game<WaitingForConnection> {
    fn from(_previous_state: &mut Game<ChoiceForPlayer>) -> Game<WaitingForConnection> {
        error_connection();
        Game {
            state: WaitingForConnection {},
        }
    }
}

impl From<&mut Game<WaitingForConnection>> for Game<ChoiceForGameStatus> {
    fn from(_previous_state: &mut Game<WaitingForConnection>) -> Game<ChoiceForGameStatus> {
        establish_connection();
        Game {
            state: ChoiceForGameStatus {},
        }
    }
}

impl From<&mut Game<ChoiceForPlayer>> for Game<Playing> {
    fn from(_previous_state: &mut Game<ChoiceForPlayer>) -> Game<Playing> {
        play();
        Game { state: Playing {} }
    }
}

impl From<&mut Game<ChoiceForPlayer>> for Game<WaitingForOpponent> {
    fn from(_previous_state: &mut Game<ChoiceForPlayer>) -> Game<WaitingForOpponent> {
        wait();
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

fn start_program() {}

fn stop_program() {}

fn display_role_selection_screen() {
}

fn display_connection_screen() {}

fn establish_connection() {}

fn start_game() {}

fn next_turn() {}

fn exit_game() {}

fn play() {}

fn wait() {}

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

    pub fn step(&mut self, event: Event) -> Self {
        match (self, event) {
            (GameWrapper::Init(previous_state), Event::AskForSelectRole) => {
                GameWrapper::SelectRole(previous_state.into())
            }
            (GameWrapper::SelectRole(previous_state), Event::SignalConnectionReady) => {
                GameWrapper::WaitingForConnection(previous_state.into())
            }
            (GameWrapper::WaitingForConnection(previous_state), Event::AskForConnection) => {
                GameWrapper::ChoiceForGameStatus(previous_state.into())
            }
            (GameWrapper::ChoiceForGameStatus(previous_state), Event::SignalToContinueTheGame) => {
                GameWrapper::ChoiceForPlayer(previous_state.into())
            }
            (GameWrapper::ChoiceForGameStatus(previous_state), Event::GameFinish) => {
                GameWrapper::WaitingForConnection(previous_state.into())
            }
            (GameWrapper::ChoiceForPlayer(previous_state), Event::PlayerTurn) => {
                GameWrapper::Playing(previous_state.into())
            }
            (GameWrapper::ChoiceForPlayer(previous_state), Event::OpponentTurn) => {
                GameWrapper::WaitingForOpponent(previous_state.into())
            }
            (GameWrapper::Playing(previous_state), Event::TurnFinish) => {
                GameWrapper::ChoiceForGameStatus(previous_state.into())
            }
            (GameWrapper::WaitingForOpponent(previous_state), Event::TurnFinish) => {
                GameWrapper::ChoiceForGameStatus(previous_state.into())
            }
            (_, Event::ErrorConnection) => {
                panic!("ErrorConnection");
            }
            (_, _) => {
                panic!("WTF !!!");
            }
        }
    }
}
