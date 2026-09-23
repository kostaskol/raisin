use std::fmt::{Debug, Display, Formatter};
const WHITE_SLOTS: std::ops::Range<usize> = 0..16;
const BLACK_SLOTS: std::ops::Range<usize> = 16..32;

#[derive(Debug)]
pub enum FenError {
    WrongFieldCount(usize),
    InvalidToMove(String),
    InvalidPieceString,
    InvalidPiece(char),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn next(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
}

impl TryFrom<char> for PieceType {
    type Error = FenError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c.to_ascii_lowercase() {
            'p' => Ok(PieceType::Pawn),
            'r' => Ok(PieceType::Rook),
            'n' => Ok(PieceType::Knight),
            'b' => Ok(PieceType::Bishop),
            'q' => Ok(PieceType::Queen),
            'k' => Ok(PieceType::King),
            _ => Err(FenError::InvalidPiece(c)),
        }
    }
}

impl PieceType {
    pub fn all() -> [PieceType; 6] {
        [
            Self::Pawn,
            Self::Rook,
            Self::Knight,
            Self::Bishop,
            Self::King,
            Self::Queen,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Square(pub usize);

impl Square {
    pub fn from_algebraic(index: &str) -> Option<Self> {
        let bytes = index.as_bytes();
        if bytes.len() != 2 {
            return None;
        }

        let file = bytes[0];
        let rank = bytes[1];

        if !(b'a'..=b'h').contains(&file) {
            return None;
        }

        if !(b'1'..=b'8').contains(&rank) {
            return None;
        }

        let file = (file - b'a') as usize + 1;
        let rank = (rank - b'1') as usize + 2;

        Some(Self::from_rank_file(rank, file))
    }

    pub fn as_algebraic(&self) -> Option<String> {
        let index = self.0;

        if index >= 120 {
            return None;
        }

        let rank_row = index / 10;
        let file_col = index % 10;

        // Within sentinel squares, outside of playable board
        if !(1..=8).contains(&file_col) || !(2..=9).contains(&rank_row) {
            return None;
        }

        let file_char = (b'a' + (file_col as u8 - 1)) as char;
        let rank_char = (b'1' + (rank_row as u8 - 2)) as char;

        Some(format!("{}{}", file_char, rank_char))
    }

    pub fn offset(&self, indx: isize) -> Self {
        Self((self.0 as isize + indx) as usize)
    }

    pub fn from_rank_file(rank: usize, file: usize) -> Self {
        Self((rank * 10) + file)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub square: Square,
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.as_symbol(),
            self.square
                .as_algebraic()
                .unwrap_or_else(|| panic!("Cannot translate {}", self.square.0))
        )
    }
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color, square: Square) -> Self {
        Self {
            piece_type,
            color,
            square,
        }
    }

    pub fn as_symbol(&self) -> char {
        let sym = match self.piece_type {
            PieceType::Pawn => 'p',
            PieceType::Rook => 'r',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        };

        match self.color {
            Color::White => sym.to_ascii_uppercase(),
            Color::Black => sym,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PieceList {
    pieces: [Option<Piece>; 32],
}

impl std::str::FromStr for PieceList {
    type Err = FenError;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        if fen.contains(' ') {
            return Err(FenError::InvalidPieceString);
        }

        let parts: Vec<&str> = fen.split("/").collect();

        if parts.len() != 8 {
            return Err(FenError::InvalidPieceString);
        }

        let mut pieces: [Option<Piece>; 32] = std::array::from_fn(|_| None);
        let mut white_index = WHITE_SLOTS.start;
        let mut black_index = BLACK_SLOTS.start;

        for (indx, &part) in parts.iter().enumerate() {
            let row = 9 - indx;
            let mut file: usize = 1;

            for c in part.chars() {
                // FEN strings start from the top of the board

                if let Some(d) = c.to_digit(10) {
                    if d == 0 {
                        return Err(FenError::InvalidPieceString);
                    }

                    file += d as usize;
                } else {
                    let square = Square::from_rank_file(row, file);
                    let piece_type = PieceType::try_from(c)?;

                    if c.is_ascii_uppercase() {
                        if !WHITE_SLOTS.contains(&white_index) {
                            return Err(FenError::InvalidPieceString);
                        }

                        pieces[white_index] = Some(Piece::new(piece_type, Color::White, square));
                        white_index += 1;
                    } else {
                        if !BLACK_SLOTS.contains(&black_index) {
                            return Err(FenError::InvalidPieceString);
                        }

                        pieces[black_index] = Some(Piece::new(piece_type, Color::Black, square));
                        black_index += 1;
                    };

                    file += 1;
                }
            }

            if file != 9 {
                return Err(FenError::InvalidPieceString);
            }
        }

        Ok(Self { pieces })
    }
}

impl PieceList {
    pub fn at(&self, index: usize) -> &Option<Piece> {
        &self.pieces[index]
    }

    pub fn white(&self) -> impl Iterator<Item = &Piece> + '_ {
        self.pieces[WHITE_SLOTS].iter().flatten()
    }

    pub fn black(&self) -> impl Iterator<Item = &Piece> + '_ {
        self.pieces[BLACK_SLOTS].iter().flatten()
    }

    pub fn all_pieces(&self) -> impl Iterator<Item = &Piece> + '_ {
        self.white().chain(self.black())
    }

    pub fn as_board(&self) -> [Option<usize>; 120] {
        let mut squares: [Option<usize>; 120] = [None; 120];

        for (indx, piece) in self.pieces.iter().enumerate() {
            if let Some(p) = piece {
                squares[p.square.0] = Some(indx);
            }
        }

        squares
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod color_tests {
        use super::*;

        #[test]
        fn next_returns_the_opposite_color() {
            assert_eq!(Color::White.next(), Color::Black);
            assert_eq!(Color::Black.next(), Color::White);
        }
    }

    mod piece_type_tests {
        use super::*;

        #[test]
        fn all_returns_all_piece_types() {
            assert_eq!(
                PieceType::all(),
                [
                    PieceType::Pawn,
                    PieceType::Rook,
                    PieceType::Knight,
                    PieceType::Bishop,
                    PieceType::King,
                    PieceType::Queen,
                ]
            )
        }
    }

    mod square_tests {
        use super::*;

        #[test]
        fn from_algebraic_with_valid_algebraic_returns_valid_index() {
            assert_eq!(Square::from_algebraic("a1"), Some(Square(21)));
            assert_eq!(Square::from_algebraic("h8"), Some(Square(98)));
        }

        #[test]
        fn from_algebraic_with_invalid_algebraic_returns_none() {
            assert_eq!(Square::from_algebraic("a9"), None);
            assert_eq!(Square::from_algebraic("z1"), None);
            assert_eq!(Square::from_algebraic("z12"), None);
        }

        #[test]
        fn as_algebraic_with_valid_index_returns_valid_algebraic() {
            assert_eq!(Square::as_algebraic(&Square(21)), Some("a1".into()));
            assert_eq!(Square::as_algebraic(&Square(98)), Some("h8".into()));
        }

        #[test]
        fn as_algebraic_with_invalid_index_returns_none() {
            assert_eq!(Square::as_algebraic(&Square(20)), None);
            assert_eq!(Square::as_algebraic(&Square(100)), None);
        }

        #[test]
        fn translating_from_and_back_as_algebraic_is_identity() {
            let alg = "f5";
            assert_eq!(
                Square::as_algebraic(&Square::from_algebraic(alg).unwrap()),
                Some(alg.into())
            );
        }
    }

    mod piece_tests {
        use super::*;
        use std::collections::HashMap;

        #[test]
        fn as_symbol_symbolises_properly() {
            let mapping = HashMap::from([
                (PieceType::Pawn, 'p'),
                (PieceType::Rook, 'r'),
                (PieceType::Knight, 'n'),
                (PieceType::Bishop, 'b'),
                (PieceType::Queen, 'q'),
                (PieceType::King, 'k'),
            ]);

            for (piece_type, symbol) in mapping {
                let piece = Piece::new(piece_type, Color::Black, Square(0));

                assert_eq!(piece.as_symbol(), symbol);
            }
        }

        #[test]
        fn as_symbol_respects_color() {
            let white_piece = Piece::new(PieceType::Pawn, Color::White, Square(0));
            assert_eq!(white_piece.as_symbol(), 'P'); // uppercase for white

            let black_piece = Piece::new(PieceType::Pawn, Color::Black, Square(0));
            assert_eq!(black_piece.as_symbol(), 'p'); // lowercase for black
        }
    }

    mod piece_list_tests {
        use super::*;

        #[test]
        fn white_returns_all_white_pieces() {
            // TODO: Fill it (and create one for black) using the new FEN parsing
        }
    }
}
