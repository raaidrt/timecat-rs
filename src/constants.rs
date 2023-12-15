pub mod description {
    pub const ENGINE_NAME: &str = "Timecat";
    pub const ENGINE_AUTHOR: &str = "Gourab Ghosh";
    pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");
}

pub mod types {
    pub type Ply = usize;
    pub type Depth = i8;
    pub type Score = i16;
    pub type MoveWeight = i64;
    pub type NumMoves = u16;
    pub type CompressedObject = u16;
    pub type Spin = u128;
    pub type ColoredStringFunction = fn(colored::ColoredString) -> colored::ColoredString;
}

pub mod bitboard {
    use crate::{paste, BitBoard, File::*};

    pub const BB_EMPTY: BitBoard = BitBoard(0);
    pub const BB_ALL: BitBoard = BitBoard(0xffff_ffff_ffff_ffff);

    macro_rules! generate_bitboard_constants {
        (@bb_squares $(($file:expr, $rank:expr)), *,) => {
            paste! {
                $(
                    pub const [<BB_$file$rank>]: BitBoard = BitBoard(1 << (8 * ($rank - 1) + $file as usize));
                )*
                pub const BB_SQUARES: [BitBoard; 64] = [$( [<BB_$file$rank>] ), *];
            }
        };

        (@bb_ranks_and_files $(($file:expr, $rank:expr)), *,) => {
            $(
                paste!{
                    pub const [<BB_RANK_$rank>]: BitBoard = BitBoard(0xff << (($rank - 1) << 3));
                    pub const [<BB_FILE_$file>]: BitBoard = BitBoard(0x0101_0101_0101_0101 << ($rank - 1));
                }
            )*
        };
    }

    #[rustfmt::skip]
    generate_bitboard_constants!(
        @bb_squares
        (A, 1), (B, 1), (C, 1), (D, 1), (E, 1), (F, 1), (G, 1), (H, 1),
        (A, 2), (B, 2), (C, 2), (D, 2), (E, 2), (F, 2), (G, 2), (H, 2),
        (A, 3), (B, 3), (C, 3), (D, 3), (E, 3), (F, 3), (G, 3), (H, 3),
        (A, 4), (B, 4), (C, 4), (D, 4), (E, 4), (F, 4), (G, 4), (H, 4),
        (A, 5), (B, 5), (C, 5), (D, 5), (E, 5), (F, 5), (G, 5), (H, 5),
        (A, 6), (B, 6), (C, 6), (D, 6), (E, 6), (F, 6), (G, 6), (H, 6),
        (A, 7), (B, 7), (C, 7), (D, 7), (E, 7), (F, 7), (G, 7), (H, 7),
        (A, 8), (B, 8), (C, 8), (D, 8), (E, 8), (F, 8), (G, 8), (H, 8),
    );
    generate_bitboard_constants!(
        @bb_ranks_and_files
        (A, 1), (B, 2), (C, 3), (D, 4), (E, 5), (F, 6), (G, 7), (H, 8),
    );

    pub const BB_CORNERS: BitBoard = BitBoard(BB_A1.0 | BB_H1.0 | BB_A8.0 | BB_H8.0);
    pub const BB_CENTER: BitBoard = BitBoard(BB_D4.0 | BB_E4.0 | BB_D5.0 | BB_E5.0);

    pub const BB_LIGHT_SQUARES: BitBoard = BitBoard(0x55aa_55aa_55aa_55aa);
    pub const BB_DARK_SQUARES: BitBoard = BitBoard(0xaa55_aa55_aa55_aa55);

    pub const BB_BACKRANKS: BitBoard = BitBoard(BB_RANK_1.0 | BB_RANK_8.0);

    pub const BB_UPPER_HALF_BOARD: BitBoard = BitBoard(0xffffffff00000000);
    pub const BB_LOWER_HALF_BOARD: BitBoard = BitBoard(0x00000000ffffffff);
    pub const BB_LEFT_HALF_BOARD: BitBoard = BitBoard(0xf0f0f0f0f0f0f0f0);
    pub const BB_RIGHT_HALF_BOARD: BitBoard = BitBoard(0x0f0f0f0f0f0f0f0f);

    pub const CENTER_SQUARES_BB: BitBoard = BitBoard(0x0000001818000000);
    pub const PSEUDO_CENTER_SQUARES_BB: BitBoard = BitBoard(0x00003C24243C0000);

    pub const UPPER_BOARD_MASK: [[BitBoard; 8]; 2] = [
        [
            BitBoard(0xffff_ffff_ffff_ff00),
            BitBoard(0xffff_ffff_ffff_0000),
            BitBoard(0xffff_ffff_ff00_0000),
            BitBoard(0xffff_ffff_0000_0000),
            BitBoard(0xffff_ff00_0000_0000),
            BitBoard(0xffff_0000_0000_0000),
            BitBoard(0xff00_0000_0000_0000),
            BitBoard(0x0000_0000_0000_0000),
        ],
        [
            BitBoard(0x00ff_ffff_ffff_ffff),
            BitBoard(0x0000_ffff_ffff_ffff),
            BitBoard(0x0000_00ff_ffff_ffff),
            BitBoard(0x0000_0000_ffff_ffff),
            BitBoard(0x0000_0000_00ff_ffff),
            BitBoard(0x0000_0000_0000_ffff),
            BitBoard(0x0000_0000_0000_00ff),
            BitBoard(0x0000_0000_0000_0000),
        ],
    ];

    pub const BOARD_QUARTER_MASKS: [BitBoard; 4] = [
        BitBoard(0x0f0f_0f0f_0000_0000),
        BitBoard(0xf0f0_f0f0_0000_0000),
        BitBoard(0x0000_0000_0f0f_0f0f),
        BitBoard(0x0000_0000_f0f0_f0f0),
    ];
}

pub mod square {
    macro_rules! generate_squares {
        [$( $square:ident ), *,] => {
            pub const SQUARES_180: [chess::Square; 64] = [$( chess::Square::$square ), *];
        };
    }

    #[rustfmt::skip]
    generate_squares![
        A8, B8, C8, D8, E8, F8, G8, H8,
        A7, B7, C7, D7, E7, F7, G7, H7,
        A6, B6, C6, D6, E6, F6, G6, H6,
        A5, B5, C5, D5, E5, F5, G5, H5,
        A4, B4, C4, D4, E4, F4, G4, H4,
        A3, B3, C3, D3, E3, F3, G3, H3,
        A2, B2, C2, D2, E2, F2, G2, H2,
        A1, B1, C1, D1, E1, F1, G1, H1,
    ];
}

pub mod board_representation {
    pub const BOARD_SKELETON: &str = r"

     A   B   C   D   E   F   G   H
   +---+---+---+---+---+---+---+---+
 8 | O | O | O | O | O | O | O | O | 8
   +---+---+---+---+---+---+---+---+
 7 | O | O | O | O | O | O | O | O | 7
   +---+---+---+---+---+---+---+---+
 6 | O | O | O | O | O | O | O | O | 6
   +---+---+---+---+---+---+---+---+
 5 | O | O | O | O | O | O | O | O | 5
   +---+---+---+---+---+---+---+---+
 4 | O | O | O | O | O | O | O | O | 4
   +---+---+---+---+---+---+---+---+
 3 | O | O | O | O | O | O | O | O | 3
   +---+---+---+---+---+---+---+---+
 2 | O | O | O | O | O | O | O | O | 2
   +---+---+---+---+---+---+---+---+
 1 | O | O | O | O | O | O | O | O | 1
   +---+---+---+---+---+---+---+---+
     A   B   C   D   E   F   G   H

";

    pub const PIECE_SYMBOLS: [&str; 7] = [" ", "p", "n", "b", "r", "q", "k"];
    pub const WHITE_PIECE_UNICODE_SYMBOLS: [&str; 6] = ["♙", "♘", "♗", "♖", "♕", "♔"];
    pub const BLACK_PIECE_UNICODE_SYMBOLS: [&str; 6] = ["♟", "♞", "♝", "♜", "♛", "♚"];
    pub const EMPTY_SPACE_UNICODE_SYMBOL: &str = " ";
}

pub mod fen {
    pub const EMPTY_FEN: &str = "8/8/8/8/8/8/8/8 w - - 0 1";
    pub const STARTING_POSITION_FEN: &str =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
}

pub mod print_style {
    use crate::ColoredStringFunction;

    macro_rules! generate_constants {
        ($constant_name:ident, [$( $func_name:ident ), *]) => {
            pub const $constant_name: &[ColoredStringFunction] = &[$( colored::Colorize::$func_name ), *];
        };
    }

    generate_constants!(WHITE_PIECES_STYLE, [white, bold]);
    generate_constants!(BLACK_PIECES_STYLE, [purple, bold]);
    generate_constants!(BOARD_SKELETON_STYLE, [green]);
    generate_constants!(BOARD_LABEL_STYLE, [red, bold]);
    generate_constants!(INFO_MESSAGE_STYLE, [bright_cyan, bold]);
    generate_constants!(CHECK_STYLE, [on_bright_red]);
    generate_constants!(CHECKERS_STYLE, [bright_red, bold]);
    generate_constants!(CHECKMATE_SCORE_STYLE, [bright_red, bold]);
    generate_constants!(PERFT_MOVE_STYLE, [green, bold]);
    generate_constants!(PERFT_COUNT_STYLE, []);
    generate_constants!(INPUT_MESSAGE_STYLE, [blue, bold]);
    generate_constants!(SUCCESS_MESSAGE_STYLE, [green, bold]);
    generate_constants!(ERROR_MESSAGE_STYLE, [red, bold]);
    generate_constants!(LAST_MOVE_HIGHLIGHT_STYLE, [on_bright_black]);
    generate_constants!(WARNING_MESSAGE_STYLE, [bright_yellow, bold]);
}

pub mod engine_constants {
    use super::types::*;
    use crate::{
        evaluate_piece, CacheTableSize, Duration, GoCommand, Piece::*, TranspositionTableEntry,
        UCIOptionValues,
    };

    pub const DEFAULT_SELFPLAY_COMMAND: GoCommand = GoCommand::from_millis(3000);
    pub const NUM_THREADS_UCI: UCIOptionValues<usize> = UCIOptionValues::new(1, 1, 1024);
    pub const T_TABLE_SIZE_UCI: UCIOptionValues<CacheTableSize> = UCIOptionValues::new(
        CacheTableSize::Exact(16),
        CacheTableSize::Exact(1),
        CacheTableSize::Exact({
            let transposition_table_entry_size =
                CacheTableSize::get_entry_size::<TranspositionTableEntry>();
            let evaluator_entry_size = CacheTableSize::get_entry_size::<Score>();
            let max_size = if transposition_table_entry_size > evaluator_entry_size {
                transposition_table_entry_size
            } else {
                evaluator_entry_size
            };
            (usize::MAX >> 21) / max_size // Assuming that Evaluator and Transposition Table will take same amount of space, so 21 not 20.
        }),
    );
    pub const MOVE_OVERHEAD_UCI: UCIOptionValues<Duration> = UCIOptionValues::new(
        Duration::from_millis(100),
        Duration::from_secs(0),
        Duration::MAX,
    );
    pub const DEFAULT_USE_OWN_BOOK: bool = false;
    pub const DEFAULT_DEBUG_MODE: bool = true;

    pub const MAX_PLY: usize = 255;
    pub const DRAW_SCORE: Score = PAWN_VALUE / 2;
    pub const CHECKMATE_SCORE: Score = 25_000;
    pub const CHECKMATE_THRESHOLD: Score = CHECKMATE_SCORE - MAX_PLY as Score - 1;
    pub const INFINITY: Score = CHECKMATE_SCORE + 4 * MAX_PLY as Score;
    pub const NUM_KILLER_MOVES: usize = 3;
    pub const PAWN_VALUE: Score = 100;
    pub const CLEAR_TABLE_AFTER_EACH_SEARCH: bool = true;

    pub const DISABLE_ALL_PRUNINGS: bool = false;
    pub const DISABLE_LMR: bool = false || DISABLE_ALL_PRUNINGS;
    pub const DISABLE_T_TABLE: bool = false || DISABLE_ALL_PRUNINGS;

    pub const NULL_MOVE_MIN_DEPTH: Depth = 2;
    pub const NULL_MOVE_MIN_REDUCTION: Depth = 2;
    pub const NULL_MOVE_DEPTH_DIVIDER: Depth = 4;

    pub const FULL_DEPTH_SEARCH_LMR: usize = 4;
    pub const REDUCTION_LIMIT_LMR: Depth = 3;
    pub const LMR_BASE_REDUCTION: f64 = 0.75;
    pub const LMR_MOVE_DIVIDER: f64 = 2.25;

    pub const ASPIRATION_WINDOW_CUTOFF: Score = PAWN_VALUE / 2;
    pub const MAX_MOVES_PER_POSITION: usize = 250;
    pub const ENDGAME_PIECE_THRESHOLD: u32 = 12;

    pub const EVALUATOR_SIZE: CacheTableSize = CacheTableSize::Exact(16);

    pub const FOLLOW_PV: bool = true;
    pub const PRINT_MOVE_INFO_DURATION_THRESHOLD: Duration = Duration::from_millis(1000);
    pub const COMMUNICATION_CHECK_INTERVAL: Duration = Duration::from_millis(100);

    pub const INITIAL_MATERIAL_SCORE_ABS: Score = 16 * PAWN_VALUE
        + 4 * (evaluate_piece(Knight) + evaluate_piece(Bishop) + evaluate_piece(Rook))
        + 2 * evaluate_piece(Queen);
    pub const MAX_MATERIAL_SCORE: Score = INITIAL_MATERIAL_SCORE_ABS / 2;
    pub const WINNING_SCORE_THRESHOLD: Score = 15 * PAWN_VALUE;

    #[rustfmt::skip]
    pub const MVV_LVA: [[MoveWeight; 6]; 6] = [
        [105, 205, 305, 405, 505, 605],
        [104, 204, 304, 404, 504, 604],
        [103, 203, 303, 403, 503, 603],
        [102, 202, 302, 402, 502, 602],
        [101, 201, 301, 401, 501, 601],
        [100, 200, 300, 400, 500, 600],
    ];

    pub const LMR_TABLE: [[Depth; 64]; 64] = [[0; 64]; 64];
}

pub mod atomic {
    pub const MEMORY_ORDERING: std::sync::atomic::Ordering = std::sync::atomic::Ordering::Relaxed;
}
