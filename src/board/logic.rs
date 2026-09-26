use super::types::{CastlingRights, Color, FenError, Piece, PieceList, PieceType, Square, Target};

const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct Board {
    // 10x12 board representation
    pub squares: [Option<usize>; 120],
    pub pieces: PieceList,
    pub to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant_target: Option<Square>,
    pub halfsteps: u32,
    pub fullsteps: u32,
}

impl std::str::FromStr for Board {
    type Err = FenError;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = fen.split_whitespace().collect();

        let [
            pieces,
            to_move,
            castling_rights,
            en_passant,
            halfsteps,
            fullsteps,
        ] = parts[..]
        else {
            return Err(FenError::WrongFieldCount(parts.len()));
        };

        let pieces = pieces.parse::<PieceList>()?;
        let invalid_clock_err = |_| FenError::InvalidClock;

        let mut board = Self {
            squares: pieces.as_board(),
            pieces,
            to_move: to_move.parse()?,
            castling_rights: castling_rights.parse()?,
            en_passant_target: None,
            halfsteps: halfsteps.parse().map_err(invalid_clock_err)?,
            fullsteps: fullsteps.parse().map_err(invalid_clock_err)?,
        };

        board.en_passant_target = board.parse_en_passant(en_passant)?;

        Ok(board)
    }
}

impl Default for Board {
    fn default() -> Self {
        str::parse::<Self>(DEFAULT_FEN).expect("Default FEN construction error")
    }
}

impl Board {
    pub fn print(&self) {
        for rank in (2..=9).rev() {
            print!("{} ", rank - 1);
            for file in 1..=8 {
                let index = Square::from_rank_file(rank, file);

                if let Some(piece_index) = self.squares[index.0] {
                    let piece = self.pieces.at(piece_index);
                    print!("{}", piece.unwrap().as_symbol());
                } else {
                    print!("*")
                }
            }

            println!();
        }

        println!("  abcdefgh");
    }

    pub fn classify_target(&self, square: &Square, color: Color) -> Target {
        if Self::is_sentinel(square) {
            return Target::Blocked;
        }

        match self.is_occupied(square) {
            Some(c) if c == color => Target::Blocked,
            Some(_) => Target::Capture,
            None => Target::Empty,
        }
    }

    pub fn piece_at(&self, square: &Square) -> Option<&Piece> {
        self.squares[square.0].and_then(|indx| self.pieces.at(indx).as_ref())
    }

    fn is_sentinel(square: &Square) -> bool {
        let rank = square.0 / 10;
        let file = square.0 % 10;

        !(1..=8).contains(&file) || !(2..=9).contains(&rank)
    }

    fn is_occupied(&self, square: &Square) -> Option<Color> {
        self.piece_at(square).map(|e| e.color)
    }

    fn parse_en_passant(&self, en_passant: &str) -> Result<Option<Square>, FenError> {
        if en_passant == "-" {
            return Ok(None);
        }

        let invalid = || FenError::InvalidEnPassant(en_passant.to_owned());
        let square = Square::from_algebraic(en_passant).ok_or_else(invalid)?;

        let direction = match self.to_move {
            Color::White => 1,
            Color::Black => -1,
        };

        let is_valid = square.is_en_passant_row(self.to_move.next())
            && self.classify_target(&square, self.to_move) == Target::Empty
            && self
                .piece_at(&square.offset(-10 * direction))
                .is_some_and(|p| p.color != self.to_move && p.piece_type == PieceType::Pawn);

        if is_valid {
            Ok(Some(square))
        } else {
            Err(invalid())
        }
    }
}

#[cfg(test)]
mod tests;
