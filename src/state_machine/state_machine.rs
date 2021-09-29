enum GameWrapper {
    None,
    Init(Game<Init>),
    WaitingForConnection(Game<WaitingForConnection>),
    ChoiceForGameStatus(Game<ChoiceForGameStatus>),
    ChoiceForPlayer(Game<ChoiceForPlayer>),
    Playing(Game<Playing>),
    WaitingForOpponent(Game<WaitingForOpponent>),
}

pub enum Event {
    Initialized,
    AskForConnection,
    ContinueGame,
    Playing,
    Waiting,
    EndTurn,
    ExitGame,
    Error,
}

#[derive(Debug)]
struct Init{}

#[derive(Debug)]
struct WaitingForConnection{}

#[derive(Debug)]
struct ChoiceForGameStatus{}

#[derive(Debug)]
struct ChoiceForPlayer{}

#[derive(Debug)]
struct WaitingForOpponent{}

#[derive(Debug)]
struct Playing{}

#[derive(Debug)]
struct Game<State> {
    state: State,
}


impl From<&mut Game<Init>> for Game<WaitingForConnection> {
    fn from(previous_state: &mut Game<Init>) -> Game<WaitingForConnection> {
        println!("last state is {:?}", previous_state);
        Game {
            state: WaitingForConnection{}
        }
    }
}

impl From<&mut Game<ChoiceForGameStatus>> for Game<ChoiceForPlayer> {
    fn from(previous_state: &mut Game<ChoiceForGameStatus>) -> Game<ChoiceForPlayer> {
        println!("last state is {:?}", previous_state);
        Game {
            state: ChoiceForPlayer{}
        }
    }
}

impl From<&mut Game<ChoiceForGameStatus>> for Game<WaitingForConnection> {
    fn from(previous_state: &mut Game<ChoiceForGameStatus>) -> Game<WaitingForConnection> {
        println!("last state is {:?}", previous_state);
        Game {
            state: WaitingForConnection{}
        }
    }
}

impl From<&mut Game<ChoiceForPlayer>> for Game<WaitingForConnection> {
    fn from(previous_state: &mut Game<ChoiceForPlayer>) -> Game<WaitingForConnection> {
        println!("last state is {:?}", previous_state);
        Game {
            state: WaitingForConnection{}
        }
    }
}

impl From<&mut Game<WaitingForConnection>> for Game<ChoiceForGameStatus> {
    fn from(previous_state: &mut Game<WaitingForConnection>) -> Game<ChoiceForGameStatus> {
        println!("last state is {:?}", previous_state);
        Game {
            state: ChoiceForGameStatus{}
        }
    }
}

impl From<&mut Game<ChoiceForPlayer>> for Game<Playing> {
    fn from(previous_state: &mut Game<ChoiceForPlayer>) -> Game<Playing> {
        println!("last state is {:?}", previous_state);
        Game {
            state: Playing{}
        }
    }
}

impl From<&mut Game<ChoiceForPlayer>> for Game<WaitingForOpponent> {
    fn from(previous_state: &mut Game<ChoiceForPlayer>) -> Game<WaitingForOpponent> {
        println!("last state is {:?}", previous_state);
        Game {
            state: WaitingForOpponent{}
        }
    }
}

impl From<&mut Game<Playing>> for Game<ChoiceForGameStatus> {
    fn from(previous_state: &mut Game<Playing>) -> Game<ChoiceForGameStatus> {
        println!("last state is {:?}", previous_state);
        Game {
            state: ChoiceForGameStatus{}
        }
    }
}

impl From<&mut Game<WaitingForOpponent>> for Game<ChoiceForGameStatus> {
    fn from(previous_state: &mut Game<WaitingForOpponent>) -> Game<ChoiceForGameStatus> {
        println!("last state is {:?}", previous_state);
        Game {
            state: ChoiceForGameStatus{}
        }
    }
}

impl Game<Init> {
    pub fn new() -> Self {
        Game {
            state: Init{}
        }
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
            (GameWrapper::ChoiceForGameStatus(previous_state), Event::ContinueGame) => {
                GameWrapper::ChoiceForPlayer(previous_state.into())
            }
            (GameWrapper::ChoiceForGameStatus(previous_state), Event::ExitGame) => {
                GameWrapper::WaitingForConnection(previous_state.into())
            }
            (GameWrapper::ChoiceForPlayer(previous_state), Event::Playing) => {
                GameWrapper::Playing(previous_state.into())
            }
            (GameWrapper::ChoiceForPlayer(previous_state), Event::Waiting) => {
                GameWrapper::WaitingForOpponent(previous_state.into())
            }
            (GameWrapper::Playing(previous_state), Event::EndTurn) => {
                GameWrapper::ChoiceForGameStatus(previous_state.into())
            }
            (GameWrapper::WaitingForOpponent(previous_state), Event::EndTurn) => {
                GameWrapper::ChoiceForGameStatus(previous_state.into())
            }
            (_, Event::Error) => {
                panic!("Error");
            }
            (_, _) => {
                panic!("WTF !!!");
            }
        }
    }
}

static mut STATE_MACHINE: GameWrapper = GameWrapper::None;

pub fn init() {
    unsafe {
        STATE_MACHINE = GameWrapper::new();
    }
}

pub fn event(event: Event) {
    unsafe {
        STATE_MACHINE = STATE_MACHINE.step(event);
    }
}
