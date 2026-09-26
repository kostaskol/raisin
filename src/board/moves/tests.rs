/*
Priority 1: moves.rs — stub modules already exist (move_tests, move_behaviour_tests at moves.rs:153-159), just empty.

- Knight from center square (e.g. d4) generates all 8 targets on an empty board
- Knight from a corner (a1) generates fewer targets — edge clipping
- King: same shape as knight, one-square version
- Rook slide: stops at board edge; stops at (includes) first enemy piece; stops before (excludes) first own piece
- Bishop: same slide rules, diagonal
- Queen: union of rook + bishop behavior
- Pawn single push from a non-start rank
- Pawn double push only from the start rank
- Pawn push blocked by an occupied square directly ahead (including: double push blocked by piece on the second square, not just the first)
- Pawn diagonal capture only when an enemy piece is there
- Pawn does not capture own piece
- Pawn on the a-file or h-file doesn't wrap a capture around to the other side of the board
- Worth testing specifically because it may expose a bug: a pawn pushing forward from the last rank (rank 8 for white). Nothing in apply_pawn (moves.rs:99-119) checks board edges before generating a push — pushes only checks is_occupied, not Board::is_sentinel. Write the test; if it returns a move onto the sentinel row, that's a real bug to flag, not something to guess about.

*/
use super::*;

mod move_tests {
    use super::*;

    mod moves_of_type_tests {
        use super::*;
        use std::collections::HashSet;

        #[test]
        fn moves_of_type_generates_all_valid_knight_moves() {
            let fen = "kK6/8/8/8/3N4/8/8/8 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Knight);

            let expected_moves =
                moves_from_algebraic("d4", &["c6", "c2", "e2", "e6", "b5", "b3", "f5", "f3"]);

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_generates_all_valid_knight_moves_when_blocked() {
            let fen = "NKk5/1Q6/8/8/8/8/8/8 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Knight);

            let expected_moves = moves_from_algebraic("a8", &["c7", "b6"]);

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_generates_all_valid_bishop_moves() {
            let fen = "k1K5/8/8/8/4B3/8/8/8 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Bishop);

            let expected_moves = moves_from_algebraic(
                "e4",
                &[
                    "d3", "c2", "b1", "f5", "g6", "h7", "f3", "g2", "h1", "d5", "c6", "b7", "a8",
                ],
            );

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_generates_all_valid_bishop_moves_when_blocked() {
            let fen = "k1K5/q7/1B6/2Q5/8/8/8/8 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Bishop);

            let expected_moves = moves_from_algebraic("b6", &["a7", "a5", "c7", "d8"]);

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_generates_all_valid_rook_moves() {
            let fen = "k1K5/8/8/8/4R3/8/8/8 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Rook);

            let expected_moves = moves_from_algebraic(
                "e4",
                &[
                    "e3", "e2", "e1", "e5", "e6", "e7", "e8", "d4", "c4", "b4", "a4", "f4", "g4",
                    "h4",
                ],
            );

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_generates_all_valid_rook_moves_when_blocked() {
            let fen = "k1K5/8/8/4q3/3QR3/8/8/8 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Rook);

            let expected_moves =
                moves_from_algebraic("e4", &["e3", "e2", "e1", "e5", "f4", "g4", "h4"]);

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_generates_all_valid_queen_moves() {
            let fen = "k1K5/8/8/8/3Q4/8/8/8 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Queen);

            let expected_moves = moves_from_algebraic(
                "d4",
                &[
                    "d5", "d6", "d7", "d8", "d3", "d2", "d1", "c4", "b4", "a4", "e4", "f4", "g4",
                    "h4", "c3", "b2", "a1", "c5", "b6", "a7", "e5", "f6", "g7", "h8", "e3", "f2",
                    "g1",
                ],
            );

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_generates_all_valid_queen_moves_when_blocked() {
            let fen = "k1K5/8/8/8/2nQ4/3N4/8/8 w KQkq - 0 0";
            let actual_moves: HashSet<_> = generate_moves_for(fen, PieceType::Queen);

            let expected_moves = moves_from_algebraic(
                "d4",
                &[
                    "d5", "d6", "d7", "d8", "c4", "e4", "f4", "g4", "h4", "c3", "b2", "a1", "c5",
                    "b6", "a7", "e5", "f6", "g7", "h8", "e3", "f2", "g1",
                ],
            );

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_generates_all_valid_king_moves() {
            let fen = "k7/8/8/8/4K3/8/8/8 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::King);

            let expected_moves =
                moves_from_algebraic("e4", &["d3", "d4", "d5", "e3", "e5", "f3", "f4", "f5"]);

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_clips_king_moves_at_the_corner() {
            let fen = "k7/8/8/8/8/8/8/K7 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::King);

            let expected_moves = moves_from_algebraic("a1", &["a2", "b1", "b2"]);

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_generates_all_valid_king_moves_when_blocked() {
            let fen = "k7/8/8/3pP3/4K3/8/8/8 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::King);

            let expected_moves =
                moves_from_algebraic("e4", &["d3", "d4", "d5", "e3", "f3", "f4", "f5"]);

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_generates_a_single_pawn_push_off_the_start_rank() {
            let fen = "k1K5/8/8/8/8/4P3/8/8 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Pawn);

            let expected_moves = moves_from_algebraic("e3", &["e4"]);

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_generates_a_double_pawn_push_from_the_start_rank() {
            let fen = "k1K5/8/8/8/8/8/4P3/8 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Pawn);

            let expected_moves = moves_from_algebraic("e2", &["e3", "e4"]);

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_generates_no_pawn_push_when_the_first_square_is_blocked() {
            let fen = "k1K5/8/8/8/8/4p3/4P3/8 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Pawn);

            let expected_moves = moves_from_algebraic("e2", &[]);

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_generates_a_single_pawn_push_when_the_second_square_is_blocked() {
            let fen = "k1K5/8/8/8/4p3/8/4P3/8 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Pawn);

            let expected_moves = moves_from_algebraic("e2", &["e3"]);

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_generates_pawn_captures_on_both_diagonals() {
            let fen = "k1K5/8/8/3p1p2/4P3/8/8/8 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Pawn);

            let expected_moves = moves_from_algebraic("e4", &["e5", "d5", "f5"]);

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_does_not_generate_pawn_captures_on_friendly_pieces() {
            let fen = "k1K5/8/8/3N1N2/4P3/8/8/8 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Pawn);

            let expected_moves = moves_from_algebraic("e4", &["e5"]);

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_does_not_wrap_a_pawn_capture_around_the_h_file() {
            let fen = "k1K5/8/8/n7/7P/8/8/8 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Pawn);

            let expected_moves = moves_from_algebraic("h4", &["h5"]);

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_does_not_wrap_a_pawn_capture_around_the_a_file() {
            let fen = "k1K5/8/8/7n/P7/8/8/8 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Pawn);

            let expected_moves = moves_from_algebraic("a4", &["a5"]);

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_does_not_push_a_white_pawn_off_the_last_rank() {
            let fen = "k1K4P/8/8/8/8/8/8/8 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Pawn);

            let expected_moves = moves_from_algebraic("h8", &[]);

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_does_not_push_a_black_pawn_off_the_first_rank() {
            let fen = "k1K5/8/8/8/8/8/8/7p b KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Pawn);

            let expected_moves = moves_from_algebraic("h1", &[]);

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_generates_a_double_push_for_a_black_pawn() {
            let fen = "k1K5/4p3/8/8/8/8/8/8 b KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Pawn);

            let expected_moves = moves_from_algebraic("e7", &["e6", "e5"]);

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_generates_black_pawn_captures_down_the_board() {
            let fen = "k1K5/8/8/4p3/3N1N2/8/8/8 b KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Pawn);

            let expected_moves = moves_from_algebraic("e5", &["e4", "d4", "f4"]);

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_clips_knight_moves_at_the_corner() {
            let fen = "k1K5/8/8/8/8/8/8/N7 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Knight);

            let expected_moves = moves_from_algebraic("a1", &["b3", "c2"]);

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_generates_moves_for_every_piece_of_that_type() {
            let fen = "k1K5/8/8/8/8/8/8/N6N w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Knight);

            let mut expected_moves = moves_from_algebraic("a1", &["b3", "c2"]);
            expected_moves.extend(moves_from_algebraic("h1", &["g3", "f2"]));

            assert_same_moves(actual_moves, expected_moves);
        }

        #[test]
        fn moves_of_type_generates_nothing_when_no_such_piece_exists() {
            let fen = "k1K5/8/8/8/8/8/8/8 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Rook);

            assert_same_moves(actual_moves, HashSet::new());
        }

        #[test]
        fn moves_of_type_ignores_pieces_of_the_other_color() {
            let fen = "k1K5/8/8/8/3n4/8/8/8 w KQkq - 0 0";
            let actual_moves = generate_moves_for(fen, PieceType::Knight);

            assert_same_moves(actual_moves, HashSet::new());
        }

        fn moves_from_algebraic(from: &str, targets: &[&str]) -> HashSet<Move> {
            targets
                .iter()
                .map(|&to| move_from_algebraic(from, to))
                .collect()
        }

        fn move_from_algebraic(from: &str, to: &str) -> Move {
            Move::new(
                Square::from_algebraic(from).expect("from should be valid square"),
                Square::from_algebraic(to).expect("to should be valid square"),
            )
        }

        fn generate_moves_for(fen: &str, piece_type: PieceType) -> HashSet<Move> {
            Move::moves_of_type(&from_valid_fen(fen), piece_type)
                .into_iter()
                .collect()
        }

        fn assert_same_moves(actual: HashSet<Move>, expected: HashSet<Move>) {
            let mut missing: Vec<_> = expected
                .difference(&actual)
                .map(|m| format!("{:?}", m))
                .collect();
            let mut unexpected: Vec<_> = actual
                .difference(&expected)
                .map(|m| format!("{:?}", m))
                .collect();

            missing.sort();
            unexpected.sort();

            assert!(
                missing.is_empty() && unexpected.is_empty(),
                "\n missing: {:?}\n unexpected: {:?}",
                missing,
                unexpected
            );
        }
    }

    mod is_under_attack_tests {
        use super::*;

        #[test]
        fn is_under_attack_returns_true_if_the_square_is_under_attack() {
            assert!(square_under_attack(
                "k1K5/8/R7/8/8/8/8/8 w KQkq - 0 0",
                "a8",
                Color::Black
            ));
        }

        #[test]
        fn is_under_attack_detects_a_bishop_on_a_diagonal() {
            assert!(square_under_attack(
                "k7/8/8/8/4B3/8/8/7K w KQkq - 0 0",
                "a8",
                Color::Black
            ));
        }

        #[test]
        fn is_under_attack_detects_a_knight() {
            assert!(square_under_attack(
                "k7/8/1N6/8/8/8/8/7K w KQkq - 0 0",
                "a8",
                Color::Black
            ));
        }

        #[test]
        fn is_under_attack_detects_a_queen_on_a_diagonal() {
            assert!(square_under_attack(
                "k1K5/8/8/8/4Q3/8/8/8 w KQkq - 0 0",
                "a8",
                Color::Black
            ));
        }

        #[test]
        fn is_under_attack_detects_a_queen_on_a_file() {
            assert!(square_under_attack(
                "k1K5/8/Q7/8/8/8/8/8 w KQkq - 0 0",
                "a8",
                Color::Black
            ));
        }

        #[test]
        fn is_under_attack_detects_an_adjacent_king() {
            assert!(square_under_attack(
                "k7/1K6/8/8/8/8/8/8 w KQkq - 0 0",
                "a8",
                Color::Black
            ));
        }

        #[test]
        fn is_under_attack_detects_a_pawn_capture_square() {
            assert!(square_under_attack(
                "k7/1P6/8/8/8/8/8/7K w KQkq - 0 0",
                "a8",
                Color::Black
            ));
        }

        #[test]
        fn is_under_attack_ignores_the_square_a_pawn_pushes_to() {
            assert!(!square_under_attack(
                "k7/P7/8/8/8/8/8/7K w KQkq - 0 0",
                "a8",
                Color::Black
            ));
        }

        #[test]
        fn is_under_attack_is_false_when_a_friendly_piece_blocks_the_ray() {
            assert!(!square_under_attack(
                "k1K5/p7/R7/8/8/8/8/8 w KQkq - 0 0",
                "a8",
                Color::Black
            ));
        }

        #[test]
        fn is_under_attack_is_false_for_friendly_pieces() {
            assert!(!square_under_attack(
                "k1K5/8/r7/8/8/8/8/8 w KQkq - 0 0",
                "a8",
                Color::Black
            ));
        }

        #[test]
        fn is_under_attack_is_false_when_nothing_attacks_the_square() {
            assert!(!square_under_attack(
                "k7/8/8/8/8/8/8/7K w KQkq - 0 0",
                "d4",
                Color::Black
            ));
        }

        fn square_under_attack(fen: &str, square: &str, defender: Color) -> bool {
            Move::is_under_attack(
                &from_valid_fen(fen),
                Square::from_algebraic(square).expect("square should be valid"),
                defender,
            )
        }
    }

    mod is_is_check_tests {
        use super::*;

        #[test]
        fn is_in_check_is_true_when_a_rook_attacks_the_king() {
            assert!(in_check("k1K5/8/R7/8/8/8/8/8 w KQkq - 0 0", Color::Black));
        }

        #[test]
        fn is_in_check_is_true_when_a_knight_attacks_the_king() {
            assert!(in_check("k7/8/1N6/8/8/8/8/7K w KQkq - 0 0", Color::Black));
        }

        #[test]
        fn is_in_check_is_true_when_a_black_pawn_attacks_a_white_king() {
            assert!(in_check("k7/8/8/8/8/8/3p4/4K3 w KQkq - 0 0", Color::White));
        }

        #[test]
        fn is_in_check_is_false_when_the_ray_is_blocked() {
            assert!(!in_check("k1K5/p7/R7/8/8/8/8/8 w KQkq - 0 0", Color::Black));
        }

        #[test]
        fn is_in_check_is_false_when_a_pawn_attacks_away_from_the_king() {
            assert!(!in_check("k7/8/8/8/4K3/3p4/8/8 w KQkq - 0 0", Color::White));
        }

        #[test]
        fn is_in_check_is_false_when_no_piece_attacks_the_king() {
            let fen = "k7/8/8/8/8/8/8/7K w KQkq - 0 0";

            assert!(!in_check(fen, Color::White));
            assert!(!in_check(fen, Color::Black));
        }

        fn in_check(fen: &str, color: Color) -> bool {
            Move::is_in_check(&from_valid_fen(fen), color)
        }
    }

    fn from_valid_fen(fen: &str) -> Board {
        str::parse::<Board>(fen).expect("valid fen produces valid board")
    }
}
