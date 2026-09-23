use super::types::{Color, FenError, Piece, PieceList, Square};

pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0";
const FEN_BOARD_INDX: usize = 0;
const FEN_TO_MOVE_INDX: usize = 1;

pub struct Board {
    // 10x12 board representation
    pub squares: [Option<usize>; 120],
    pub pieces: PieceList,
    pub to_move: Color,
}

impl std::str::FromStr for Board {
    type Err = FenError;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        // Ok(Self::on_initial_positions())
        let parts: Vec<&str> = fen.split(" ").collect();

        if parts.len() != 6 {
            return Err(FenError::WrongFieldCount(parts.len()));
        }

        let to_move = match parts[FEN_TO_MOVE_INDX] {
            "w" => Color::White,
            "b" => Color::Black,
            m => return Err(FenError::InvalidToMove(m.to_owned())),
        };

        let piece_list = parts[FEN_BOARD_INDX].parse::<PieceList>()?;
        Ok(Self {
            squares: piece_list.as_board(),
            pieces: piece_list,
            to_move,
        })
    }
}

impl Board {
    pub fn print(&self) {
        for rank in (2..=9).rev() {
            for file in (1..=8) {
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
    }

    pub fn is_valid_square(&self, square: &Square) -> bool {
        if Self::is_sentinel(square) {
            return false;
        }

        if let Some(occupied_color) = self.is_occupied(square) {
            return occupied_color != self.to_move;
        }

        true
    }

    pub fn is_sentinel(square: &Square) -> bool {
        let rank = square.0 / 10;
        let file = square.0 % 10;

        !(1..=8).contains(&file) || !(2..=9).contains(&rank)
    }

    pub fn is_occupied(&self, square: &Square) -> Option<Color> {
        self.piece_at(square).map(|e| e.color)
    }

    pub fn piece_at(&self, square: &Square) -> Option<&Piece> {
        self.squares[square.0].and_then(|indx| self.pieces.at(indx).as_ref())
    }

    fn populate_squares(pieces: &PieceList) -> [Option<usize>; 120] {
        let mut squares: [Option<usize>; 120] = std::array::from_fn(|_| None);

        for (index, piece) in pieces.all_pieces().enumerate() {
            squares[piece.square.0] = Some(index);
        }

        squares
    }
}
