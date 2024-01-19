#![allow(dead_code)]

mod board;
mod chess;
mod constants;
mod engine;
mod evaluate;
mod nnue;
mod nnue_rs;
mod parse;
mod polyglot;
mod search;
mod selfplay;
mod sort;
mod syzygy;
mod tests;
mod timer;
mod tt;
mod uci;
mod useful_macros;
mod utils;

use arrayvec::ArrayVec;
pub use board::*;
pub use chess::*;
use constants::atomic::*;
use constants::bitboard::*;
use constants::color::*;
use constants::board_representation::*;
use constants::description::*;
use constants::engine_constants::*;
pub use constants::fen::*;
use constants::print_style::*;
use constants::square::*;
pub use constants::types::*;
use engine::{Engine, GoCommand};
use evaluate::*;
use failure::Fail;
pub use fxhash::FxHashMap as HashMap;
pub use itertools::Itertools;
use lazy_static::lazy_static;
use nodrop::NoDrop;
pub use parse::*;
pub use paste::paste;
pub use polyglot::*;
use search::*;
pub use selfplay::self_play;
use sort::*;
use std::cmp::Ordering;
use std::convert::From;
use std::env;
use std::fmt;
use std::fs;
use std::mem::{self, transmute};
use std::num::ParseIntError;
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, IndexMut, Mul, MulAssign, Not, Shl,
    ShlAssign, Shr, ShrAssign, Add, AddAssign, Sub, SubAssign,
};
use std::str::{FromStr, ParseBoolError};
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
// pub use syzygy::*;
use std::error::Error;
use tests::test;
use timer::Timer;
use tt::*;
use uci::*;
pub use utils::bitboard_utils::*;
pub use utils::cache_table_utils::*;
pub use utils::classes::*;
pub use utils::color::*;
pub use utils::engine_error::*;
pub use utils::engine_utils::*;
pub use utils::global_utils::*;
pub use utils::hash_utils::*;
pub use utils::info_utils::*;
pub use utils::io_utils::*;
pub use utils::move_utils::*;
pub use utils::piece_utils::*;
pub use utils::pv_utils::*;
pub use utils::square_utils::*;
pub use utils::string_utils::*;
pub use utils::time_utils::*;

// pub use std::hint;
// pub use std::num;

lazy_static! {
    pub static ref TRANSPOSITION_TABLE: TranspositionTable = TranspositionTable::default();
    pub static ref EVALUATOR: Evaluator = Evaluator::default();
    pub static ref IO_READER: IoReader = IoReader::default();
    pub static ref UCI_OPTIONS: UCIOptionsVec = UCIOptionsVec::default();
}
