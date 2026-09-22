use std::fmt::{Debug, Display, Formatter};
const WHITE_SLOTS: std::ops::Range<usize> = 0..16;
const BLACK_SLOTS: std::ops::Range<usize> = 16..32;

#[derive(Debug)]
pub enum FenError {
    WrongFieldCount(usize),
    InvalidToMove(String),
    InvalidPieceString,
    InvalidPiece(char),
    InvalidBoardState,
    InvalidClock,
    InvalidCastlingRights(String),
    InvalidEnPassant(String),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

impl std::str::FromStr for Color {
    type Err = FenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Self::White),
            "b" => Ok(Self::Black),
            _ => Err(FenError::InvalidToMove(s.to_owned())),
        }
    }
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

    pub fn promotions() -> [PieceType; 4] {
        [Self::Rook, Self::Knight, Self::Bishop, Self::Queen]
    }

    pub fn as_symbol(&self) -> char {
        match self {
            PieceType::Pawn => 'p',
            PieceType::Rook => 'r',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Blocked,
    Capture,
    Empty,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CastlingRights(u8);

impl std::str::FromStr for CastlingRights {
    type Err = FenError;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        if fen == "-" {
            return Ok(Self::NONE);
        }

        let mut rights = Self::NONE;

        for c in fen.chars() {
            let right = match c {
                'K' => Self::WHITE_KING,
                'Q' => Self::WHITE_QUEEN,
                'k' => Self::BLACK_KING,
                'q' => Self::BLACK_QUEEN,
                _ => return Err(FenError::InvalidCastlingRights(fen.to_owned())),
            };

            if rights.contains(right) {
                return Err(FenError::InvalidCastlingRights(fen.to_owned()));
            }

            rights.insert(right);
        }

        Ok(rights)
    }
}

impl CastlingRights {
    pub const NONE: Self = Self(0);
    pub const WHITE_KING: Self = Self(0b0001);
    pub const WHITE_QUEEN: Self = Self(0b0010);
    pub const BLACK_KING: Self = Self(0b0100);
    pub const BLACK_QUEEN: Self = Self(0b1000);

    pub fn contains(&self, rights: Self) -> bool {
        self.0 & rights.0 == rights.0
    }

    pub fn insert(&mut self, rights: Self) {
        self.0 |= rights.0;
    }

    pub fn remove(&mut self, rights: Self) {
        self.0 &= !rights.0;
    }

    pub fn for_color(color: Color) -> Self {
        match color {
            Color::White => Self(Self::WHITE_KING.0 | Self::WHITE_QUEEN.0),
            Color::Black => Self(Self::BLACK_KING.0 | Self::BLACK_QUEEN.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Square(pub usize);

impl Square {
    const WHITE_LAST_ROW: usize = 9;
    const WHITE_PAWN_START_ROW: usize = 3;
    const BLACK_LAST_ROW: usize = 2;
    const BLACK_PAWN_START_ROW: usize = 8;
    const WHITE_EN_PASSANT_ROW: usize = 4;
    const BLACK_EN_PASSANT_ROW: usize = 7;

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

    pub fn row(&self) -> usize {
        self.0 / 10
    }

    pub fn file(&self) -> usize {
        self.0 % 10
    }

    pub fn is_pawn_first_row(&self, color: Color) -> bool {
        self.row()
            == match color {
                Color::White => Self::WHITE_PAWN_START_ROW,
                Color::Black => Self::BLACK_PAWN_START_ROW,
            }
    }

    pub fn is_last_rank(&self, color: Color) -> bool {
        self.row()
            == match color {
                Color::White => Self::WHITE_LAST_ROW,
                Color::Black => Self::BLACK_LAST_ROW,
            }
    }

    pub fn is_en_passant_row(&self, color: Color) -> bool {
        self.row()
            == match color {
                Color::White => Self::WHITE_EN_PASSANT_ROW,
                Color::Black => Self::BLACK_EN_PASSANT_ROW,
            }
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
        match self.color {
            Color::White => self.piece_type.as_symbol().to_ascii_uppercase(),
            Color::Black => self.piece_type.as_symbol(),
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

            let mut prev_digit = false;

            for c in part.chars() {
                // FEN strings start from the top of the board

                if let Some(d) = c.to_digit(10) {
                    if d == 0 || prev_digit {
                        return Err(FenError::InvalidPieceString);
                    }

                    prev_digit = true;

                    file += d as usize;
                } else {
                    prev_digit = false;
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

        let piece_list = Self { pieces };

        if piece_list.is_valid_state() {
            Ok(Self { pieces })
        } else {
            Err(FenError::InvalidBoardState)
        }
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

    pub fn is_valid_state(&self) -> bool {
        self.black()
            .filter(|&p| p.piece_type == PieceType::King)
            .count()
            == 1
            && self
                .white()
                .filter(|&p| p.piece_type == PieceType::King)
                .count()
                == 1
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
        use std::collections::HashMap;

        const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

        #[test]
        fn default_fen_string_is_parsed_correctly() {
            str::parse::<PieceList>(DEFAULT_FEN)
                .expect("default FEN string parsing should not fail");
        }

        #[test]
        fn default_fen_creates_the_correct_pieces() {
            let piece_list = from_valid_fen(DEFAULT_FEN);

            assert!(piece_list.pieces.iter().all(|p| p.is_some()));

            let assertions = HashMap::from([
                (PieceType::Pawn, 8usize),
                (PieceType::Rook, 2usize),
                (PieceType::Knight, 2usize),
                (PieceType::Bishop, 2usize),
                (PieceType::King, 1usize),
                (PieceType::Queen, 1usize),
            ]);

            for (piece_type, expected_count) in assertions {
                assert_eq!(
                    piece_of_color_count(&piece_list, piece_type, Color::White),
                    expected_count
                );
                assert_eq!(
                    piece_of_color_count(&piece_list, piece_type, Color::Black),
                    expected_count
                );
            }
        }

        #[test]
        fn default_fen_places_all_pieces_correctly() {
            let piece_list = from_valid_fen(DEFAULT_FEN);

            let assertions = [
                (PieceType::Pawn, Color::White, "a2"),
                (PieceType::Pawn, Color::White, "b2"),
                (PieceType::Pawn, Color::White, "c2"),
                (PieceType::Pawn, Color::White, "d2"),
                (PieceType::Pawn, Color::White, "e2"),
                (PieceType::Pawn, Color::White, "f2"),
                (PieceType::Pawn, Color::White, "g2"),
                (PieceType::Pawn, Color::White, "h2"),
                (PieceType::Rook, Color::White, "a1"),
                (PieceType::Rook, Color::White, "h1"),
                (PieceType::Knight, Color::White, "b1"),
                (PieceType::Knight, Color::White, "g1"),
                (PieceType::Bishop, Color::White, "c1"),
                (PieceType::Bishop, Color::White, "f1"),
                (PieceType::Queen, Color::White, "d1"),
                (PieceType::King, Color::White, "e1"),
                (PieceType::Pawn, Color::Black, "a7"),
                (PieceType::Pawn, Color::Black, "b7"),
                (PieceType::Pawn, Color::Black, "c7"),
                (PieceType::Pawn, Color::Black, "d7"),
                (PieceType::Pawn, Color::Black, "e7"),
                (PieceType::Pawn, Color::Black, "f7"),
                (PieceType::Pawn, Color::Black, "g7"),
                (PieceType::Pawn, Color::Black, "h7"),
                (PieceType::Rook, Color::Black, "a8"),
                (PieceType::Rook, Color::Black, "h8"),
                (PieceType::Knight, Color::Black, "b8"),
                (PieceType::Knight, Color::Black, "g8"),
                (PieceType::Bishop, Color::Black, "c8"),
                (PieceType::Bishop, Color::Black, "f8"),
                (PieceType::Queen, Color::Black, "d8"),
                (PieceType::King, Color::Black, "e8"),
            ];

            for (piece_type, color, alg_square) in assertions {
                assert_eq!(
                    piece_at(&piece_list, alg_square),
                    Some(Piece::new(
                        piece_type,
                        color,
                        Square::from_algebraic(alg_square).unwrap()
                    ))
                );
            }
        }

        #[test]
        fn white_returns_all_white_pieces() {
            let piece_list = from_valid_fen("8/8/8/8/8/8/PPRNK2k/8");

            assert_eq!(piece_list.white().count(), 5);
        }

        #[test]
        fn black_returns_all_black_pieces() {
            let piece_list = from_valid_fen("8/pprnk2K/8/8/8/8/8/8");

            assert_eq!(piece_list.black().count(), 5);
        }

        #[test]
        fn non_default_fen_strings_parse_correctly() {
            let piece_list = from_valid_fen("8/8/7n/3k4/7K/8/6R1/8");

            assert_eq!(piece_list.white().count(), 2);
            assert_eq!(piece_list.black().count(), 2);
            assert_eq!(
                piece_at(&piece_list, "d5"),
                Some(Piece::new(
                    PieceType::King,
                    Color::Black,
                    Square::from_algebraic("d5").unwrap(),
                )),
            );
        }

        #[test]
        fn too_low_rank_length_fen_produces_error() {
            let invalid_fen = "8";
            let piece_list = str::parse::<PieceList>(invalid_fen);

            assert!(matches!(piece_list, Err(FenError::InvalidPieceString)));
        }

        #[test]
        fn too_high_rank_length_fen_produces_error() {
            let invalid_fen = "8/8/8/8/8/8/8/8/8";
            let piece_list = str::parse::<PieceList>(invalid_fen);

            assert!(matches!(piece_list, Err(FenError::InvalidPieceString)));
        }

        #[test]
        fn too_low_file_length_fen_produces_error() {
            let invalid_fen = "8/8/7/8/8/8/8/8";
            let piece_list = str::parse::<PieceList>(invalid_fen);

            assert!(matches!(piece_list, Err(FenError::InvalidPieceString)));
        }

        #[test]
        fn too_high_file_length_fen_produces_error() {
            let invalid_fen = "8/8/9/8/8/8/8/8";
            let piece_list = str::parse::<PieceList>(invalid_fen);

            assert!(matches!(piece_list, Err(FenError::InvalidPieceString)));
        }

        #[test]
        fn bad_file_length_with_trailing_piece_produces_error() {
            let invalid_fen = "8/8/8p/8/8/8/8/8";
            let piece_list = str::parse::<PieceList>(invalid_fen);

            assert!(matches!(piece_list, Err(FenError::InvalidPieceString)));
        }

        #[test]
        fn fen_containing_spaces_produces_error() {
            let invalid_fen = "8/8/8/8/8/8/8/8 w";
            let piece_list = str::parse::<PieceList>(invalid_fen);

            assert!(matches!(piece_list, Err(FenError::InvalidPieceString)));
        }

        #[test]
        fn fen_containing_zeroes_produces_error() {
            let invalid_fen = "8/8/8/8/7r0/8/8/8";
            let piece_list = str::parse::<PieceList>(invalid_fen);

            assert!(matches!(piece_list, Err(FenError::InvalidPieceString)));
        }

        #[test]
        fn fen_containing_invalid_piece_types_produces_error() {
            let invalid_fen = "8/8/8/8/8/7x/8/8";
            let piece_list = str::parse::<PieceList>(invalid_fen);

            assert!(matches!(piece_list, Err(FenError::InvalidPiece('x'))));
        }

        #[test]
        fn fen_containing_too_many_white_pieces_produces_error() {
            let invalid_fen = "PPPPPPPP/PPPPPPPP/PPPPPPPP/8/8/8/8/8";
            let piece_list = str::parse::<PieceList>(invalid_fen);

            assert!(matches!(piece_list, Err(FenError::InvalidPieceString)));
        }

        #[test]
        fn fen_containing_too_many_black_pieces_produces_error() {
            let invalid_fen = "pppppppp/pppppppp/pppppppp/8/8/8/8/8";
            let piece_list = str::parse::<PieceList>(invalid_fen);

            assert!(matches!(piece_list, Err(FenError::InvalidPieceString)));
        }

        #[test]
        fn is_valid_state_is_false_when_board_is_missing_a_king() {
            let mut missing_white_king_pieces = [None; 32];
            missing_white_king_pieces[BLACK_SLOTS.start] =
                Some(Piece::new(PieceType::King, Color::Black, Square(24)));

            let piece_list = PieceList {
                pieces: missing_white_king_pieces,
            };

            assert!(!piece_list.is_valid_state());

            let mut missing_black_king_pieces = [None; 32];
            missing_black_king_pieces[WHITE_SLOTS.start] =
                Some(Piece::new(PieceType::King, Color::White, Square(24)));

            let piece_list = PieceList {
                pieces: missing_black_king_pieces,
            };

            assert!(!piece_list.is_valid_state());
        }

        fn piece_of_color_count(
            piece_list: &PieceList,
            piece_type: PieceType,
            color: Color,
        ) -> usize {
            piece_list
                .pieces
                .iter()
                .flatten()
                .filter(|&p| p.piece_type == piece_type && p.color == color)
                .count()
        }

        fn piece_at(piece_list: &PieceList, alg_square: &str) -> Option<Piece> {
            let square =
                Square::from_algebraic(alg_square).expect("valid square in algebraic form");
            let index = piece_list.as_board()[square.0]?;
            *piece_list.at(index)
        }

        fn from_valid_fen(fen: &str) -> PieceList {
            str::parse::<PieceList>(fen).expect("valid FEN to be parsed")
        }
    }
}
