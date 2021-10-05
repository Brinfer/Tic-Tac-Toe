///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Public
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Init and create the state machine
pub fn new() {
    unsafe {
        STATE_MACHINE = GameWrapper::new();
    }
}

/// Destroy the state machine
pub fn free() {
    unsafe {
        STATE_MACHINE = GameWrapper::None;
    }
}

pub fn ask_for_connection() {
    unsafe {
        STATE_MACHINE = STATE_MACHINE.step(Event::AskForConnection);
    }
}

pub fn signal_to_continue_the_game() {
    unsafe {
        STATE_MACHINE = STATE_MACHINE.step(Event::SignalToContinueTheGame);
    }
}

pub fn signal_to_play() {
    unsafe {
        STATE_MACHINE = STATE_MACHINE.step(Event::PlayerTurn);
    }
}

pub fn ask_for_wait_opponent() {
    unsafe {
        STATE_MACHINE = STATE_MACHINE.step(Event::OpponentTurn);
    }
}

pub fn signal_finish_turn() {
    unsafe {
        STATE_MACHINE = STATE_MACHINE.step(Event::TurnFinish);
    }
}

pub fn signal_game_finish() {
    unsafe {
        STATE_MACHINE = STATE_MACHINE.step(Event::GameFinish);
    }
}

pub fn error_connection() {
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
    Initialized,
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
    WaitingForConnection(Game<WaitingForConnection>),
    ChoiceForGameStatus(Game<ChoiceForGameStatus>),
    ChoiceForPlayer(Game<ChoiceForPlayer>),
    Playing(Game<Playing>),
    WaitingForOpponent(Game<WaitingForOpponent>),
}

#[derive(Debug)]
struct Init {}

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

impl From<&mut Game<Init>> for Game<WaitingForConnection> {
    fn from(previous_state: &mut Game<Init>) -> Game<WaitingForConnection> {
        println!("last state is {:?}", previous_state);
        Game {
            state: WaitingForConnection {},
        }
    }
}

impl From<&mut Game<ChoiceForGameStatus>> for Game<ChoiceForPlayer> {
    fn from(previous_state: &mut Game<ChoiceForGameStatus>) -> Game<ChoiceForPlayer> {
        println!("last state is {:?}", previous_state);
        Game {
            state: ChoiceForPlayer {},
        }
    }
}

impl From<&mut Game<ChoiceForGameStatus>> for Game<WaitingForConnection> {
    fn from(previous_state: &mut Game<ChoiceForGameStatus>) -> Game<WaitingForConnection> {
        println!("last state is {:?}", previous_state);
        Game {
            state: WaitingForConnection {},
        }
    }
}

impl From<&mut Game<ChoiceForPlayer>> for Game<WaitingForConnection> {
    fn from(previous_state: &mut Game<ChoiceForPlayer>) -> Game<WaitingForConnection> {
        println!("last state is {:?}", previous_state);
        Game {
            state: WaitingForConnection {},
        }
    }
}

impl From<&mut Game<WaitingForConnection>> for Game<ChoiceForGameStatus> {
    fn from(previous_state: &mut Game<WaitingForConnection>) -> Game<ChoiceForGameStatus> {
        println!("last state is {:?}", previous_state);
        Game {
            state: ChoiceForGameStatus {},
        }
    }
}

impl From<&mut Game<ChoiceForPlayer>> for Game<Playing> {
    fn from(previous_state: &mut Game<ChoiceForPlayer>) -> Game<Playing> {
        println!("last state is {:?}", previous_state);
        Game { state: Playing {} }
    }
}

impl From<&mut Game<ChoiceForPlayer>> for Game<WaitingForOpponent> {
    fn from(previous_state: &mut Game<ChoiceForPlayer>) -> Game<WaitingForOpponent> {
        println!("last state is {:?}", previous_state);
        Game {
            state: WaitingForOpponent {},
        }
    }
}

impl From<&mut Game<Playing>> for Game<ChoiceForGameStatus> {
    fn from(previous_state: &mut Game<Playing>) -> Game<ChoiceForGameStatus> {
        println!("last state is {:?}", previous_state);
        Game {
            state: ChoiceForGameStatus {},
        }
    }
}

impl From<&mut Game<WaitingForOpponent>> for Game<ChoiceForGameStatus> {
    fn from(previous_state: &mut Game<WaitingForOpponent>) -> Game<ChoiceForGameStatus> {
        println!("last state is {:?}", previous_state);
        Game {
            state: ChoiceForGameStatus {},
        }
    }
}

//////////////////////////////////////////// Actions //////////////////////////////////////////////////////////////////

fn start_program() {}

fn stop_program() {}

fn display_connection_screen() {}

fn etablish_connection() {}

fn start_game() {}

fn next_turn() {}

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
            (GameWrapper::Init(previous_state), Event::Initialized) => {
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
