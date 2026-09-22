use super::*;

mod board_tests {
    use super::*;

    mod from_str_tests {
        use super::*;

        #[test]
        fn a_valid_fen_is_parsed() {
            let board = from_valid_fen("k1K5/8/8/8/8/8/8/8 w KQkq - 12 34");

            assert_eq!(board.to_move, Color::White);
            assert_eq!(board.en_passant_target, None);
            assert_eq!(board.halfsteps, 12);
            assert_eq!(board.fullsteps, 34);
        }

        #[test]
        fn too_few_fields_are_rejected() {
            assert!(matches!(
                parse("k1K5/8/8/8/8/8/8/8 w KQkq - 0"),
                Err(FenError::WrongFieldCount(5))
            ));
        }

        #[test]
        fn too_many_fields_are_rejected() {
            assert!(matches!(
                parse("k1K5/8/8/8/8/8/8/8 w KQkq - 0 0 0"),
                Err(FenError::WrongFieldCount(7))
            ));
        }

        #[test]
        fn surrounding_whitespace_is_tolerated() {
            assert!(parse("k1K5/8/8/8/8/8/8/8 w KQkq - 0 0\n").is_ok());
            assert!(parse("k1K5/8/8/8/8/8/8/8  w  KQkq - 0 0").is_ok());
        }

        #[test]
        fn both_colors_to_move_are_parsed() {
            assert_eq!(
                from_valid_fen("k1K5/8/8/8/8/8/8/8 w - - 0 0").to_move,
                Color::White
            );
            assert_eq!(
                from_valid_fen("k1K5/8/8/8/8/8/8/8 b - - 0 0").to_move,
                Color::Black
            );
        }

        #[test]
        fn an_invalid_color_to_move_is_rejected() {
            assert!(matches!(
                parse("k1K5/8/8/8/8/8/8/8 x - - 0 0"),
                Err(FenError::InvalidToMove(_))
            ));
        }

        #[test]
        fn all_castling_rights_are_parsed() {
            let rights = from_valid_fen("k1K5/8/8/8/8/8/8/8 w KQkq - 0 0").castling_rights;

            assert!(rights.contains(CastlingRights::WHITE_KING));
            assert!(rights.contains(CastlingRights::WHITE_QUEEN));
            assert!(rights.contains(CastlingRights::BLACK_KING));
            assert!(rights.contains(CastlingRights::BLACK_QUEEN));
        }

        #[test]
        fn partial_castling_rights_are_parsed() {
            let rights = from_valid_fen("k1K5/8/8/8/8/8/8/8 w Kq - 0 0").castling_rights;

            assert!(rights.contains(CastlingRights::WHITE_KING));
            assert!(rights.contains(CastlingRights::BLACK_QUEEN));
            assert!(!rights.contains(CastlingRights::WHITE_QUEEN));
            assert!(!rights.contains(CastlingRights::BLACK_KING));
        }

        #[test]
        fn absent_castling_rights_are_parsed() {
            assert_eq!(
                from_valid_fen("k1K5/8/8/8/8/8/8/8 w - - 0 0").castling_rights,
                CastlingRights::NONE
            );
        }

        #[test]
        fn an_unknown_castling_symbol_is_rejected() {
            assert!(matches!(
                parse("k1K5/8/8/8/8/8/8/8 w KQkqX - 0 0"),
                Err(FenError::InvalidCastlingRights(_))
            ));
        }

        #[test]
        fn a_repeated_castling_symbol_is_rejected() {
            assert!(matches!(
                parse("k1K5/8/8/8/8/8/8/8 w KK - 0 0"),
                Err(FenError::InvalidCastlingRights(_))
            ));
        }

        #[test]
        fn a_non_numeric_halfstep_clock_is_rejected() {
            assert!(matches!(
                parse("k1K5/8/8/8/8/8/8/8 w - - x 0"),
                Err(FenError::InvalidClock)
            ));
        }

        #[test]
        fn a_non_numeric_fullstep_clock_is_rejected() {
            assert!(matches!(
                parse("k1K5/8/8/8/8/8/8/8 w - - 0 x"),
                Err(FenError::InvalidClock)
            ));
        }

        #[test]
        fn a_missing_king_is_rejected() {
            assert!(matches!(
                parse("8/8/8/8/8/8/8/8 w - - 0 0"),
                Err(FenError::InvalidBoardState)
            ));
        }

        #[test]
        fn a_duplicate_king_is_rejected() {
            assert!(matches!(
                parse("k1K5/8/8/8/8/8/8/K7 w - - 0 0"),
                Err(FenError::InvalidBoardState)
            ));
        }

        #[test]
        fn pieces_are_reachable_by_square() {
            let board = from_valid_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0");

            let white_king = board.piece_at(&square("e1")).expect("white king on e1");
            assert_eq!(white_king.piece_type, PieceType::King);
            assert_eq!(white_king.color, Color::White);

            let black_rook = board.piece_at(&square("h8")).expect("black rook on h8");
            assert_eq!(black_rook.piece_type, PieceType::Rook);
            assert_eq!(black_rook.color, Color::Black);

            assert!(board.piece_at(&square("d4")).is_none());
        }

        #[test]
        fn adjacent_digits_in_the_board_field_are_rejected() {
            assert!(parse("k1K5/8/44/8/8/8/8/8 w - - 0 0").is_err());
        }
    }

    mod parse_en_passant_tests {
        use super::*;

        #[test]
        fn a_dash_means_no_target() {
            assert_eq!(
                from_valid_fen("k1K5/8/8/8/8/8/8/8 w - - 0 0").en_passant_target,
                None
            );
        }

        #[test]
        fn a_valid_target_for_white_is_parsed() {
            assert_eq!(
                from_valid_fen("k1K5/8/8/3pP3/8/8/8/8 w - d6 0 0").en_passant_target,
                Some(square("d6"))
            );
        }

        #[test]
        fn a_valid_target_for_black_is_parsed() {
            assert_eq!(
                from_valid_fen("k1K5/8/8/8/3Pp3/8/8/8 b - d3 0 0").en_passant_target,
                Some(square("d3"))
            );
        }

        #[test]
        fn a_target_without_a_pawn_behind_it_is_rejected() {
            assert!(matches!(
                parse("k1K5/8/8/4P3/8/8/8/8 w - d6 0 0"),
                Err(FenError::InvalidEnPassant(_))
            ));
        }

        #[test]
        fn an_occupied_target_is_rejected() {
            assert!(matches!(
                parse("k1K5/8/3n4/4P3/8/8/8/8 w - d6 0 0"),
                Err(FenError::InvalidEnPassant(_))
            ));
        }

        #[test]
        fn a_target_on_the_wrong_row_is_rejected() {
            assert!(matches!(
                parse("k1K5/8/8/4P3/8/8/8/8 w - d3 0 0"),
                Err(FenError::InvalidEnPassant(_))
            ));
        }

        #[test]
        fn a_target_that_is_not_a_square_is_rejected() {
            assert!(matches!(
                parse("k1K5/8/8/4P3/8/8/8/8 w - zz 0 0"),
                Err(FenError::InvalidEnPassant(_))
            ));
        }

        #[test]
        fn a_friendly_pawn_behind_the_target_is_rejected() {
            assert!(matches!(
                parse("k1K5/8/8/3Pp3/8/8/8/8 b - d6 0 0"),
                Err(FenError::InvalidEnPassant(_))
            ));
        }
    }

    mod classify_target_tests {
        use super::*;

        const FEN: &str = "k1K5/8/8/3pP3/8/8/8/8 w - - 0 0";

        #[test]
        fn an_empty_square_is_empty() {
            assert_eq!(
                from_valid_fen(FEN).classify_target(&square("d4"), Color::White),
                Target::Empty
            );
        }

        #[test]
        fn a_friendly_piece_blocks() {
            assert_eq!(
                from_valid_fen(FEN).classify_target(&square("e5"), Color::White),
                Target::Blocked
            );
        }

        #[test]
        fn an_enemy_piece_can_be_captured() {
            assert_eq!(
                from_valid_fen(FEN).classify_target(&square("d5"), Color::White),
                Target::Capture
            );
        }

        #[test]
        fn the_same_square_flips_meaning_for_the_other_color() {
            let board = from_valid_fen(FEN);

            assert_eq!(
                board.classify_target(&square("e5"), Color::Black),
                Target::Capture
            );
            assert_eq!(
                board.classify_target(&square("d5"), Color::Black),
                Target::Blocked
            );
        }

        #[test]
        fn squares_outside_the_playable_board_block() {
            let board = from_valid_fen(FEN);

            for index in [0, 19, 20, 29, 90, 99, 100, 119] {
                assert_eq!(
                    board.classify_target(&Square(index), Color::White),
                    Target::Blocked,
                    "index {} should be blocked",
                    index
                );
            }
        }
    }

    mod default_tests {
        use super::*;

        #[test]
        fn the_default_board_is_the_start_position() {
            let board = Board::default();

            assert_eq!(board.to_move, Color::White);
            assert_eq!(board.en_passant_target, None);
            assert_eq!(board.halfsteps, 0);

            let rights = board.castling_rights;
            assert!(rights.contains(CastlingRights::WHITE_KING));
            assert!(rights.contains(CastlingRights::WHITE_QUEEN));
            assert!(rights.contains(CastlingRights::BLACK_KING));
            assert!(rights.contains(CastlingRights::BLACK_QUEEN));
        }

        #[test]
        fn the_default_board_starts_on_move_one() {
            assert_eq!(Board::default().fullsteps, 1);
        }
    }

    fn parse(fen: &str) -> Result<Board, FenError> {
        str::parse::<Board>(fen)
    }

    fn from_valid_fen(fen: &str) -> Board {
        parse(fen).expect("valid fen produces valid board")
    }

    fn square(algebraic: &str) -> Square {
        Square::from_algebraic(algebraic).expect("valid algebraic square")
    }
}
