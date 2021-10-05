use std::sync::Mutex;

#[path = "../tools.rs"]
#[macro_use]
mod tools;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Public
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Init and create the state machine
pub fn new() -> StateMachine {
    info!("[Event] Create the state machine");
    return StateMachine{ currentState : Mutex::new(GameWrapper::new())};
}

/// Destroy the state machine
pub fn free(p_state_machine: &StateMachine) {
    info!("[Event] Destroy the state machine");

    {
        let mut state_machine = p_state_machine.currentState.lock().unwrap();
        *state_machine = GameWrapper::free();
    }
}

pub fn ask_for_connection(p_state_machine: &StateMachine) {
    info!("[Event] Ask for connection");

    {
        let mut state_changer = p_state_machine.currentState.lock().unwrap();
        *state_changer = (*state_changer).step(Event::AskForConnection);
    }
}

pub fn ask_for_select_role(p_state_machine: &StateMachine) {
    info!("[Event] Ask for select role");

    {
        let mut state_changer = p_state_machine.currentState.lock().unwrap();
        *state_changer = (*state_changer).step(Event::AskForSelectRole);
    }
}

pub fn signal_connection_established(p_state_machine: &StateMachine) {
    info!("[Event] Signal the connection is ethablished");

    {
        let mut state_changer = p_state_machine.currentState.lock().unwrap();
        *state_changer = (*state_changer).step(Event::SignalConnectionReady);
    }
}

pub fn signal_to_continue_the_game(p_state_machine: &StateMachine) {
    info!("[Event] Signal to continue the game");

    {
        let mut state_changer = p_state_machine.currentState.lock().unwrap();
        *state_changer = (*state_changer).step(Event::SignalToContinueTheGame);
    }
}

pub fn signal_to_play(p_state_machine: &StateMachine) {
    info!("[Event] Signal at the user to play");

    {
        let mut state_machine = p_state_machine.currentState.lock().unwrap();
        *state_machine = (*state_machine).step(Event::PlayerTurn);
    }
}

pub fn ask_for_wait_opponent(p_state_machine: &StateMachine) {
    info!("[Event] Ask to wait the opponent");

    {
        let mut state_machine = p_state_machine.currentState.lock().unwrap();
        *state_machine = (*state_machine).step(Event::OpponentTurn);
    }
}

pub fn signal_finish_turn(p_state_machine: &StateMachine) {
    info!("[Event] Signal the end of the turn");

    {
        let mut state_machine = p_state_machine.currentState.lock().unwrap();
        *state_machine = (*state_machine).step(Event::TurnFinish);
    }
}

pub fn signal_game_finish(p_state_machine: &StateMachine) {
    info!("[Event] Signal the end of the game");

    {
        let mut state_machine = p_state_machine.currentState.lock().unwrap();
        *state_machine = (*state_machine).step(Event::GameFinish);
    }
}

pub fn signal_error_connection(p_state_machine: &StateMachine) {
    info!("[Event] A connection error occur");

    {
        let mut state_machine = p_state_machine.currentState.lock().unwrap();
        *state_machine = (*state_machine).step(Event::ErrorConnection);
    }
}

pub struct StateMachine {
    currentState: Mutex<GameWrapper>,
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Private
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// The current state of the state machine
// static STATE_MACHINE: Mutex<GameWrapper> = Mutex::new(GameWrapper::None);

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
        is_my_turn();
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
        start_game();
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
        next_turn();
        Game {
            state: ChoiceForGameStatus {},
        }
    }
}

impl From<&mut Game<WaitingForOpponent>> for Game<ChoiceForGameStatus> {
    fn from(_previous_state: &mut Game<WaitingForOpponent>) -> Game<ChoiceForGameStatus> {
        next_turn();
        Game {
            state: ChoiceForGameStatus {},
        }
    }
}

//////////////////////////////////////////// Actions //////////////////////////////////////////////////////////////////

fn display_role_selection_screen() {
    info!("[Action] Display the selection screen");
}

fn display_connection_screen() {
    info!("[Action] Display the connection screen");
}

fn start_game() {
    info!("[Action] Start the game");
}

fn is_my_turn() {
    info!("[Action] Test if it's my turn");
}

fn next_turn() {
    info!("[Action] Pass to the next turn");
}

fn exit_game() {
    info!("[Action] Exit the game");
}

fn play() {
    info!("[Action] Player turn");
}

fn wait() {
    info!("[Action] Opponent turn");
}

fn error_connection() {
    info!("[Action] Error connection");
}

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

    pub fn free() -> Self {
        GameWrapper::None
    }

    pub fn step(&mut self, event: Event) -> Self {
        match (self, event) {
            (GameWrapper::Init(previous_state), Event::AskForSelectRole) => {
                GameWrapper::SelectRole(previous_state.into())
            }
            (GameWrapper::SelectRole(previous_state), Event::AskForConnection) => {
                GameWrapper::WaitingForConnection(previous_state.into())
            }
            (GameWrapper::WaitingForConnection(previous_state), Event::SignalConnectionReady) => {
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
