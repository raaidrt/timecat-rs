use super::*;
use EngineError::*;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum EngineError {
    UnknownCommand,
    NoInput,
    NotImplemented,
    ColoredOutputFeatureNotEnabled,
    EngineNotRunning,
    BadFen { fen: String },
    InvalidDepth { depth: Depth },
    IllegalMove {
        move_text: String,
        board_fen: String,
    },
    ColoredOutputUnchanged { b: bool },
    ConsoleModeUnchanged,
    EmptyStack,
    BestMoveNotFound { fen: String },
    NullMoveInCheck { fen: String },
    WTimeNotMentioned,
    BTimeNotMentioned,
    GameAlreadyOver,
    UnknownDebugCommand { command: String },
    InvalidSpinValue {
        name: String,
        value: Spin,
        min: Spin,
        max: Spin,
    },
    InvalidSanMoveString { s: String },
    InvalidRankString { s: String },
    InvalidFileString { s: String },
    InvalidSquareString { s: String },
    InvalidUciMoveString { s: String },
    InvalidSubBoard { board: SubBoard },
    CustomError { err_msg: String },
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnknownCommand => write!(f, "{}", UnknownCommand.stringify()),
            NoInput => write!(f, "No input! Please try again!"),
            NotImplemented => write!(f, "Sorry, this command is not implemented yet :("),
            ColoredOutputFeatureNotEnabled => write!(f, "Colored Output Feature is not enabled. Recompile the chess engine enabling the feature!"),
            EngineNotRunning => write!(f, "Engine is not running! Please try again!"),
            BadFen { fen } => write!(f, "Bad FEN string: {fen}! Please try Again!"),
            InvalidDepth { depth } => write!(f, "Invalid depth {depth}! Please try again!"),
            IllegalMove { move_text, board_fen } => write!(f, "Illegal move {move_text} in position {board_fen}! Please try again!"),
            ColoredOutputUnchanged { b } => write!(f, "Colored output already set to {b}! Please try again!"),
            ConsoleModeUnchanged => write!(f, "Already in Console Mode! Please try again!"),
            EmptyStack => write!(f, "Move Stack is empty, pop not possible! Please try again!"),
            BestMoveNotFound { fen } => write!(f, "Best move not found in position {fen}! Please try again!"),
            NullMoveInCheck { fen } => write!(f, "Cannot apply null move in position {fen}, as king is in check! Please try again!"),
            WTimeNotMentioned => write!(f, "You didn't mention wtime! Please try again!"),
            BTimeNotMentioned => write!(f, "You didn't mention btime! Please try again!"),
            GameAlreadyOver => write!(f, "Game is already over! Please start a game from another position!"),
            UnknownDebugCommand { command } => write!(f, "Debug command {command} is unknown! The possible commands are on or off! Please try again!"),
            InvalidSpinValue {name, value, min, max} => write!(f, "Cannot set value of {name} to {value}, the value must be from {min} to {max}! Please try again!"),
            InvalidSanMoveString { s } => write!(f, "Got invalid SAN move string {s}! Please try again!"),
            InvalidRankString { s } => write!(f, "Got invalid rank string {s}! Please try again!"),
            InvalidFileString { s } => write!(f, "Got invalid file string {s}! Please try again!"),
            InvalidSquareString { s } => write!(f, "Got invalid square string {s}! Please try again!"),
            InvalidUciMoveString { s } => write!(f, "Invalid uci move string {s}! Please try again!"),
            InvalidSubBoard { board } => write!(f, "Invalid sub board generated:\n\n{board:#?}"),
            CustomError { err_msg } => write!(f, "{err_msg}"),
        }
    }
}

impl Error for EngineError {}

impl EngineError {
    pub fn stringify_with_optional_raw_input(&self, optional_raw_input: Option<&str>) -> String {
        match self {
            Self::UnknownCommand => {
                let command_type = if UCI_STATE.is_in_console_mode() {
                    "Console"
                } else {
                    "UCI"
                };
                match optional_raw_input {
                    Some(raw_input) => format!(
                        "Unknown {command_type} Command: {:?}\nType help for more information!",
                        raw_input.trim_end_matches('\n')
                    ),
                    None => format!("Unknown {command_type} Command!\nPlease try again!"),
                }
            }
            other_err => other_err.to_string(),
        }
    }
}

impl Stringify for EngineError {
    fn stringify(&self) -> String {
        self.stringify_with_optional_raw_input(None)
    }
}

impl From<EngineError> for String {
    fn from(error: EngineError) -> Self {
        error.stringify()
    }
}

impl From<&Self> for EngineError {
    fn from(error: &Self) -> Self {
        error.clone()
    }
}

impl From<ParseBoolError> for EngineError {
    fn from(error: ParseBoolError) -> Self {
        CustomError {
            err_msg: format!("Failed to parse bool, {error}! Please try again!"),
        }
    }
}

impl From<ParseIntError> for EngineError {
    fn from(error: ParseIntError) -> Self {
        CustomError {
            err_msg: format!("Failed to parse integer, {error}! Please try again!"),
        }
    }
}

macro_rules! impl_error_convert {
    ($class:ty) => {
        impl From<$class> for EngineError {
            fn from(error: $class) -> Self {
                CustomError {
                    err_msg: format!("{error}! Please try again!"),
                }
            }
        }
    };
}

impl_error_convert!(std::io::Error);
impl_error_convert!(std::array::TryFromSliceError);

impl From<String> for EngineError {
    fn from(err_msg: String) -> Self {
        CustomError { err_msg }
    }
}

impl From<&str> for EngineError {
    fn from(err_msg: &str) -> Self {
        err_msg.to_string().into()
    }
}
