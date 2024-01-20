use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WeightedMove {
    pub weight: MoveWeight,
    pub move_: Move,
}

impl Default for WeightedMove {
    fn default() -> Self {
        Self {
            weight: 0,
            move_: Move::new(Square::A1, Square::A1, None),
        }
    }
}

impl WeightedMove {
    pub fn new(move_: Move, weight: MoveWeight) -> Self {
        Self { move_, weight }
    }
}

#[derive(Debug)]
pub struct WeightedMoveListSorter {
    weighted_moves: [WeightedMove; MAX_MOVES_PER_POSITION],
    len: usize,
    idx: usize,
    sorted: bool,
}

impl WeightedMoveListSorter {
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl Iterator for WeightedMoveListSorter {
    type Item = WeightedMove;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.len {
            self.idx = 0;
            self.sorted = true;
            return None;
        }
        if self.sorted {
            let best_move = get_item_unchecked!(self.weighted_moves, self.idx);
            self.idx += 1;
            return Some(best_move);
        }
        let mut max_weight = MoveWeight::MIN;
        let mut max_idx = self.idx;
        for idx in self.idx..self.len {
            let weighted_move = self.weighted_moves[idx];
            if weighted_move.weight > max_weight {
                max_idx = idx;
                max_weight = weighted_move.weight;
            }
        }
        // unsafe { self.weighted_moves.swap_unchecked(self.idx, max_idx) };
        self.weighted_moves.swap(self.idx, max_idx);
        let best_move = self.weighted_moves[self.idx];
        self.idx += 1;
        Some(best_move)
    }
}

impl FromIterator<WeightedMove> for WeightedMoveListSorter {
    fn from_iter<T: IntoIterator<Item = WeightedMove>>(iter: T) -> Self {
        let mut weighted_moves = [WeightedMove::default(); MAX_MOVES_PER_POSITION];
        let mut len = 0;
        let mut sorted = true;
        let mut last_weight = MoveWeight::MAX;
        for weighted_move in iter {
            weighted_moves[len] = weighted_move;
            if sorted {
                sorted = last_weight > weighted_move.weight;
                last_weight = weighted_move.weight;
            }
            len += 1;
        }
        Self {
            weighted_moves,
            len,
            idx: 0,
            sorted,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MoveSorter {
    killer_moves: [[Option<Move>; NUM_KILLER_MOVES]; MAX_PLY],
    // TODO: change this into 64 x 12 array
    history_move_scores: [[[MoveWeight; 64]; 2]; 6],
    follow_pv: bool,
    score_pv: bool,
}

impl MoveSorter {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn reset_variables(&mut self) {
        for ply in 0..MAX_PLY {
            for idx in 0..NUM_KILLER_MOVES {
                self.killer_moves[ply][idx] = None;
            }
        }
        for piece in 0..6 {
            for color in 0..2 {
                for square in 0..64 {
                    self.history_move_scores[piece][color][square] = 0;
                }
            }
        }
        self.follow_pv = false;
        self.score_pv = false;
    }

    pub fn update_killer_moves(&mut self, killer_move: Move, ply: Ply) {
        let arr = &mut self.killer_moves[ply];
        arr.rotate_right(1);
        arr[0] = Some(killer_move);
    }

    pub fn is_killer_move(&self, move_: Move, ply: Ply) -> bool {
        self.killer_moves[ply].contains(&Some(move_))
    }

    pub fn add_history_move(&mut self, history_move: Move, board: &Board, depth: Depth) {
        let depth = (depth as MoveWeight).pow(2);
        let src = history_move.get_source();
        let dest = history_move.get_dest();
        let piece = board.piece_at(src).unwrap();
        self.history_move_scores[piece.get_piece_type().to_index()]
            [piece.get_color().to_index()][dest.to_index()] += depth;
    }

    #[inline(always)]
    pub fn get_history_score(&self, history_move: Move, board: &Board) -> MoveWeight {
        let src = history_move.get_source();
        let dest = history_move.get_dest();
        let piece = board.piece_at(src).unwrap();
        self.history_move_scores[piece.get_piece_type().to_index()]
            [piece.get_color().to_index()][dest.to_index()]
    }

    fn get_least_attackers_move(square: Square, board: &SubBoard) -> Option<Move> {
        let mut capture_moves = MoveGen::new_legal(board);
        capture_moves.set_iterator_mask(get_square_bb(square));
        capture_moves.next()
    }

    fn see(square: Square, board: &SubBoard) -> Score {
        let least_attackers_move = match Self::get_least_attackers_move(square, board) {
            Some(move_) => move_,
            None => return 0,
        };
        let capture_piece = board.piece_type_at(square).unwrap_or(Pawn);
        (evaluate_piece(capture_piece)
            - Self::see(square, &board.make_move_new(least_attackers_move)))
        .max(0)
    }

    fn see_capture(square: Square, board: &SubBoard) -> Score {
        let least_attackers_move = match Self::get_least_attackers_move(square, board) {
            Some(move_) => move_,
            None => return 0,
        };
        let capture_piece = board.piece_type_at(square).unwrap_or(Pawn);
        evaluate_piece(capture_piece)
            - Self::see(square, &board.make_move_new(least_attackers_move))
    }

    fn mvv_lva(move_: Move, best_move: Option<Move>, board: &Board) -> MoveWeight {
        if best_move == Some(move_) {
            return 10000;
        }
        MVV_LVA[board.piece_type_at(move_.get_source()).unwrap().to_index()][board
            .piece_type_at(move_.get_dest())
            .unwrap_or(Pawn)
            .to_index()]
    }

    #[inline(always)]
    fn score_capture(move_: Move, best_move: Option<Move>, board: &Board) -> MoveWeight {
        if Some(move_) == best_move {
            return 10000;
        }
        Self::see_capture(move_.get_dest(), board.get_sub_board()) as MoveWeight
        // Self::mvv_lva(move_, board)
    }

    fn score_easily_winning_position_moves(
        board: &Board,
        source: Square,
        dest: Square,
    ) -> Option<MoveWeight> {
        let moving_piece = board.piece_type_at(source).unwrap();
        if moving_piece != Pawn {
            let losing_color = !board.get_winning_side().unwrap_or(White);
            let losing_king_square = board.get_king_square(losing_color);
            if losing_king_square == source {
                return Some(-100 * square_distance(source, Square::E4) as MoveWeight);
            }
            let source_distance = square_distance(source, losing_king_square);
            let dest_distance = square_distance(dest, losing_king_square);
            if dest_distance < source_distance {
                return Some(
                    50 * match moving_piece {
                        King => 5,
                        Knight => 4,
                        Queen => 3,
                        Rook => 2,
                        Bishop => 1,
                        _ => unreachable!(),
                    } - dest_distance as MoveWeight,
                );
            }
        }
        None
    }

    fn score_move(
        &mut self,
        move_: Move,
        board: &Board,
        ply: Ply,
        best_move: Option<Move>,
        pv_move: Option<Move>,
        is_easily_winning_position: bool,
    ) -> MoveWeight {
        // pv move
        if self.score_pv && pv_move == Some(move_) {
            self.score_pv = false;
            return 129000000;
        }
        // best move
        if best_move == Some(move_) {
            return 128000000;
        }
        if board.is_capture(move_) {
            return 126000000 + Self::score_capture(move_, None, board);
        }
        for (idx, &optional_move) in self.killer_moves[ply].iter().enumerate() {
            if optional_move == Some(move_) {
                return 125000000 - idx as MoveWeight;
            }
        }
        // move pieces towards the king
        let source = move_.get_source();
        let dest = move_.get_dest();
        if is_easily_winning_position {
            if let Some(score) = Self::score_easily_winning_position_moves(board, source, dest) {
                return 124000000 + score;
            }
        }
        if move_.get_promotion().is_some() {
            return 123000000;
        }
        if board.is_passed_pawn(source) {
            let promotion_distance = board
                .turn()
                .to_their_backrank()
                .to_index()
                .abs_diff(source.get_rank().to_index());
            return 122000000 - promotion_distance as MoveWeight;
        }
        let move_made_sub_board = board.get_sub_board().make_move_new(move_);
        // check
        let checkers = *move_made_sub_board.checkers();
        let moving_piece = board.piece_type_at(source).unwrap();
        if checkers != BB_EMPTY {
            return -127000000 + 10 * checkers.popcnt() as MoveWeight - moving_piece as MoveWeight;
        }
        if board.is_irreversible(move_) {
            return 121000000;
        }
        // history
        let history_score = self.get_history_score(move_, board);
        if history_score != 0 {
            return 120000000 + history_score;
        }
        MAX_MOVES_PER_POSITION as MoveWeight
            - MoveGen::new_legal(&move_made_sub_board).len() as MoveWeight
    }

    pub fn get_weighted_moves_sorted(
        &mut self,
        board: &Board,
        ply: Ply,
        optional_best_move: impl Into<Option<Move>>,
        optional_pv_move: impl Into<Option<Move>>,
        is_easily_winning_position: bool,
    ) -> WeightedMoveListSorter {
        let mut best_move = optional_best_move.into();
        if best_move.is_none() {
            best_move = TRANSPOSITION_TABLE.read_best_move(board.hash());
        }
        let optional_pv_move = optional_pv_move.into();
        let moves_vec = Vec::from_iter(board.generate_legal_moves());
        if self.follow_pv {
            self.follow_pv = false;
            if let Some(pv_move) = optional_pv_move {
                if moves_vec.contains(&pv_move) {
                    self.follow_pv = true;
                    self.score_pv = true;
                }
            }
        }
        if moves_vec.len() < 2 {
            return WeightedMoveListSorter::from_iter(
                moves_vec.iter().map(|&move_| WeightedMove::new(move_, 0)),
            );
        }
        WeightedMoveListSorter::from_iter(moves_vec.into_iter().enumerate().map(|(idx, m)| {
            WeightedMove::new(
                m,
                1000 * self.score_move(
                    m,
                    board,
                    ply,
                    best_move,
                    optional_pv_move,
                    is_easily_winning_position,
                ) + MAX_MOVES_PER_POSITION as MoveWeight
                    - idx as MoveWeight,
            )
        }))
    }

    pub fn get_weighted_capture_moves_sorted(&self, board: &Board) -> WeightedMoveListSorter {
        let best_move = TRANSPOSITION_TABLE.read_best_move(board.hash());
        WeightedMoveListSorter::from_iter(board.generate_legal_captures().enumerate().map(
            |(idx, m)| {
                WeightedMove::new(
                    m,
                    1000 * Self::score_capture(m, best_move, board)
                        + MAX_MOVES_PER_POSITION as MoveWeight
                        - idx as MoveWeight,
                )
            },
        ))
    }

    pub fn score_root_moves(
        board: &Board,
        move_: Move,
        pv_move: impl Into<Option<Move>>,
    ) -> MoveWeight {
        if Some(move_) == pv_move.into() {
            return 100_000;
        }
        if board.gives_repetition(move_) {
            return -50;
        }
        let is_endgame = board.is_endgame();
        if !is_endgame && board.gives_claimable_threefold_repetition(move_) {
            return -40;
        }
        let mut score = 0;
        let mut evaluation = board.evaluate_flipped() as MoveWeight;
        if evaluation == 0 {
            evaluation = 1;
        }
        if is_endgame {
            if move_.get_promotion().is_some() {
                score += 30000;
            }
            if board.is_capture(move_) {
                score += 2000 * evaluation.signum() + Self::score_capture(move_, None, board);
            }
            let source = move_.get_source();
            if board.is_passed_pawn(source) {
                let promotion_distance = board
                    .turn()
                    .to_their_backrank()
                    .to_index()
                    .abs_diff(source.get_rank().to_index());
                score += 20 - promotion_distance as MoveWeight;
            }
        }
        score
    }

    pub fn follow_pv(&mut self) {
        self.follow_pv = true;
    }

    pub fn is_following_pv(&self) -> bool {
        self.follow_pv
    }
}

impl Default for MoveSorter {
    fn default() -> Self {
        Self {
            killer_moves: [[None; NUM_KILLER_MOVES]; MAX_PLY],
            history_move_scores: [[[0; 64]; 2]; 6],
            follow_pv: false,
            score_pv: false,
        }
    }
}
