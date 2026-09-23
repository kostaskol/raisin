use super::logic::Board;
use super::types::{Color, Piece, PieceType, Square};

const KNIGHT_OFFSETS: [isize; 8] = [
    21, 19, 12, 8, // fwd / rside jumps
    -8, -12, -19, -21, // bwd / lside jumps
];
const KING_OFFSETS: [isize; 8] = [9, 10, 11, -9, -10, -11, 1, -1];
const ROOK_OFFSETS: [isize; 4] = [1, 10, -10, -1];
const BISHOP_OFFSETS: [isize; 4] = [11, 9, -9, -11];
// Rook + Bishop
const QUEEN_OFFSETS: [isize; 8] = [11, 9, -9, -11, 1, 10, -10, -1];

#[derive(Copy, Clone)]
enum MoveBehaviour {
    Slide,
    Step,
    Pawn,
}

impl MoveBehaviour {
    pub fn for_type(piece_type: PieceType) -> Self {
        match piece_type {
            PieceType::Rook | PieceType::Bishop | PieceType::Queen => Self::Slide,
            PieceType::Knight | PieceType::King => Self::Step,
            PieceType::Pawn => Self::Pawn,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Move {
    pub from: Square,
    pub to: Square,
}

impl Move {
    pub fn new(from: Square, to: Square) -> Self {
        Self { from, to }
    }

    pub fn generate_moves(board: &Board) -> Vec<Move> {
        PieceType::all()
            .into_iter()
            .flat_map(|e| Self::moves_of_type(board, e))
            .collect()
    }

    fn moves_of_type(board: &Board, piece_type: PieceType) -> Vec<Move> {
        let offsets = Self::offsets_for(piece_type);
        let behaviour = MoveBehaviour::for_type(piece_type);

        Self::get_all_of_type(board, piece_type)
            .into_iter()
            .flat_map(|piece| {
                match behaviour {
                    MoveBehaviour::Slide => Self::apply_slide(board, piece.square, offsets),
                    MoveBehaviour::Step => Self::apply_step(board, piece.square, offsets),
                    MoveBehaviour::Pawn => Self::apply_pawn(board, piece.square),
                }
                .into_iter()
                .map(|target_square| Move::new(piece.square, target_square))
            })
            .collect()
    }

    fn apply_step(board: &Board, square: Square, offsets: &[isize]) -> Vec<Square> {
        offsets
            .iter()
            .map(|offset| Square((square.0 as isize + offset) as usize))
            .filter(|target| board.is_valid_square(target))
            .collect()
    }

    fn apply_slide(board: &Board, square: Square, offsets: &[isize]) -> Vec<Square> {
        offsets
            .iter()
            .flat_map(|&offset| {
                (1..)
                    .map(move |step| Square((square.0 as isize + (offset * step)) as usize))
                    .scan(false, |done, target| {
                        if *done || Board::is_sentinel(&target) {
                            return None;
                        }

                        match board.is_occupied(&target) {
                            Some(c) if c == board.to_move => None,
                            Some(_) => {
                                *done = true;
                                Some(target)
                            }
                            None => Some(target),
                        }
                    })
            })
            .collect()
    }

    fn apply_pawn(board: &Board, square: Square) -> Vec<Square> {
        let (direction, start_row) = match board.to_move {
            Color::White => (1, 3),
            Color::Black => (-1, 8),
        };

        let target = |offset: isize| Square((square.0 as isize + offset * direction) as usize);

        let max_push = if square.0 / 10 == start_row { 2 } else { 1 };

        let pushes = (1..=max_push)
            .map(|step| target(step * 10))
            .take_while(|sq| board.is_occupied(sq).is_none());

        let captures = [9, 11]
            .into_iter()
            .map(target)
            .filter(|sq| board.is_occupied(sq).is_some_and(|c| c != board.to_move));

        pushes.chain(captures).collect()
    }

    fn get_all_of_type(board: &Board, piece_type: PieceType) -> Vec<&Piece> {
        if board.to_move == Color::White {
            board
                .pieces
                .white()
                .filter(|e| e.piece_type == piece_type)
                .collect()
        } else {
            board
                .pieces
                .black()
                .filter(|e| e.piece_type == piece_type)
                .collect()
        }
    }

    fn offsets_for(piece_type: PieceType) -> &'static [isize] {
        match piece_type {
            PieceType::Rook => &ROOK_OFFSETS,
            PieceType::Knight => &KNIGHT_OFFSETS,
            PieceType::Bishop => &BISHOP_OFFSETS,
            PieceType::Queen => &QUEEN_OFFSETS,
            PieceType::King => &KING_OFFSETS,
            PieceType::Pawn => &[], // Pawns have special offsets
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod move_behaviour_tests {}

    mod move_tests {
        use crate::board::Board;

        use super::*;
    }
}
