use super::logic::Board;
use super::types::{Color, Piece, PieceType, Square, Target};

const KNIGHT_OFFSETS: [isize; 8] = [
    21, 19, 12, 8, // fwd / rside jumps
    -8, -12, -19, -21, // bwd / lside jumps
];
const KING_OFFSETS: [isize; 8] = [9, 10, 11, -9, -10, -11, 1, -1];
const ROOK_OFFSETS: [isize; 4] = [1, 10, -10, -1];
const BISHOP_OFFSETS: [isize; 4] = [11, 9, -9, -11];
// Rook + Bishop
const QUEEN_OFFSETS: [isize; 8] = [11, 9, -9, -11, 1, 10, -10, -1];
const WHITE_PAWN_CAPTURE_OFFSETS: [isize; 2] = [11, 9];
const BLACK_PAWN_CAPTURE_OFFSETS: [isize; 2] = [-11, -9];

#[derive(Copy, Clone, PartialEq)]
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

    // To determine whether a square is under attack, we need
    // to treat pawns as steppers instead of them being special
    pub fn attack_for_type(piece_type: PieceType) -> Self {
        match Self::for_type(piece_type) {
            Self::Slide => Self::Slide,
            Self::Step | Self::Pawn => Self::Step,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum MoveKind {
    Normal,
    EnPassant,
    Castle,
    Promotion(PieceType),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub move_kind: MoveKind,
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}->{}",
            self.from
                .as_algebraic()
                .unwrap_or_else(|| format!("?{}", self.from.0)),
            self.to
                .as_algebraic()
                .unwrap_or_else(|| format!("?{}", self.to.0)),
        )?;

        match self.move_kind {
            MoveKind::Castle => write!(f, " O-O"),
            MoveKind::EnPassant => write!(f, " ep"),
            MoveKind::Promotion(pt) => write!(f, "{}", pt.as_symbol()),
            MoveKind::Normal => Ok(()),
        }
    }
}

impl Move {
    pub fn new(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            move_kind: MoveKind::Normal,
        }
    }

    pub fn with_kind(from: Square, to: Square, move_kind: MoveKind) -> Self {
        Self {
            from,
            to,
            move_kind,
        }
    }

    pub fn promotion(from: Square, to: Square, piece_type: PieceType) -> Self {
        Self {
            from,
            to,
            move_kind: MoveKind::Promotion(piece_type),
        }
    }

    pub fn en_passant(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            move_kind: MoveKind::EnPassant,
        }
    }

    pub fn castle(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            move_kind: MoveKind::Castle,
        }
    }

    pub fn all_promotions(from: Square, to: Square) -> Vec<Self> {
        PieceType::promotions()
            .iter()
            .map(|&pt| Self::promotion(from, to, pt))
            .collect()
    }

    pub fn generate_moves(board: &Board) -> Vec<Move> {
        PieceType::all()
            .into_iter()
            .flat_map(|e| Self::moves_of_type(board, e))
            .collect()
    }

    pub fn is_under_attack(board: &Board, square: Square, defender: Color) -> bool {
        for piece_type in PieceType::all() {
            let offsets = Self::attack_offsets_for(piece_type, defender);

            let moves = match MoveBehaviour::attack_for_type(piece_type) {
                MoveBehaviour::Step => Self::apply_step(board, square, defender, offsets),
                MoveBehaviour::Slide => Self::apply_slide(board, square, defender, offsets),
                MoveBehaviour::Pawn => unreachable!("Can't parse pawn behaviour during attack"),
            };

            if moves
                .iter()
                .filter_map(|(s, _)| board.piece_at(s))
                .any(|p| p.color != defender && p.piece_type == piece_type)
            {
                return true;
            }
        }

        false
    }

    pub fn is_in_check(board: &Board, defender: Color) -> bool {
        let king_square = board
            .pieces
            .all_pieces()
            .find(|&&p| p.color == defender && p.piece_type == PieceType::King)
            .expect("expected king to be present");

        Self::is_under_attack(board, king_square.square, defender)
    }

    fn moves_of_type(board: &Board, piece_type: PieceType) -> Vec<Move> {
        let offsets = Self::offsets_for(piece_type);
        let behaviour = MoveBehaviour::for_type(piece_type);

        Self::get_all_of_type(board, piece_type, board.to_move)
            .into_iter()
            .flat_map(|piece| {
                match behaviour {
                    MoveBehaviour::Slide => {
                        Self::apply_slide(board, piece.square, board.to_move, offsets)
                    }
                    MoveBehaviour::Step => {
                        Self::apply_step(board, piece.square, board.to_move, offsets)
                    }
                    MoveBehaviour::Pawn => Self::apply_pawn(board, piece.square, board.to_move),
                }
                .into_iter()
                .flat_map(|(target_square, move_kind)| {
                    if behaviour == MoveBehaviour::Pawn && target_square.is_last_rank(board.to_move)
                    {
                        Move::all_promotions(piece.square, target_square)
                    } else {
                        vec![Move::with_kind(piece.square, target_square, move_kind)]
                    }
                })
            })
            .collect()
    }

    fn apply_step(
        board: &Board,
        square: Square,
        color: Color,
        offsets: &[isize],
    ) -> Vec<(Square, MoveKind)> {
        offsets
            .iter()
            .map(|&offset| square.offset(offset))
            .filter(|target| board.classify_target(target, color) != Target::Blocked)
            .map(|sq| (sq, MoveKind::Normal))
            .collect()
    }

    fn apply_slide(
        board: &Board,
        square: Square,
        color: Color,
        offsets: &[isize],
    ) -> Vec<(Square, MoveKind)> {
        offsets
            .iter()
            .flat_map(|&offset| {
                (1..)
                    .map(move |step| Square((square.0 as isize + (offset * step)) as usize))
                    .scan(false, |done, target| {
                        if *done {
                            return None;
                        }

                        match board.classify_target(&target, color) {
                            Target::Blocked => None,
                            Target::Capture => {
                                *done = true;
                                Some(target)
                            }
                            Target::Empty => Some(target),
                        }
                    })
            })
            .map(|sq| (sq, MoveKind::Normal))
            .collect()
    }

    fn apply_pawn(board: &Board, square: Square, color: Color) -> Vec<(Square, MoveKind)> {
        let direction = match color {
            Color::White => 1,
            Color::Black => -1,
        };

        let target = |offset: isize| square.offset(offset * direction);

        let max_push = if square.is_pawn_first_row(color) {
            2
        } else {
            1
        };

        let pushes = (1..=max_push)
            .map(|step| target(step * 10))
            .take_while(|sq| board.classify_target(sq, color) == Target::Empty);

        let captures = [9, 11]
            .into_iter()
            .map(target)
            .filter(|sq| board.classify_target(sq, color) == Target::Capture);

        let en_passant_capture = board
            .en_passant_target
            .filter(|ts| [9, 11].into_iter().map(target).any(|sq| sq == *ts));

        pushes
            .chain(captures)
            .map(|sq| (sq, MoveKind::Normal))
            .chain(en_passant_capture.map(|sq| (sq, MoveKind::EnPassant)))
            .collect()
    }

    fn get_all_of_type(board: &Board, piece_type: PieceType, color: Color) -> Vec<&Piece> {
        if color == Color::White {
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

    fn attack_offsets_for(piece_type: PieceType, color: Color) -> &'static [isize] {
        match piece_type {
            PieceType::Rook => &ROOK_OFFSETS,
            PieceType::Knight => &KNIGHT_OFFSETS,
            PieceType::Bishop => &BISHOP_OFFSETS,
            PieceType::Queen => &QUEEN_OFFSETS,
            PieceType::King => &KING_OFFSETS,
            PieceType::Pawn => match color {
                Color::White => &WHITE_PAWN_CAPTURE_OFFSETS,
                Color::Black => &BLACK_PAWN_CAPTURE_OFFSETS,
            },
        }
    }
}

#[cfg(test)]
mod tests;
