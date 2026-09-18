mod board;

use board::{Board, DEFAULT_FEN, Move};

fn main() {
    if let Ok(board) = str::parse::<Board>(DEFAULT_FEN) {
        board.print();

        println!("Valid moves for board 2");
        let moves = Move::generate_moves(&board);

        for move_k in moves {
            let piece = board
                .pieces
                .at(board.squares[move_k.from.0].unwrap())
                .unwrap();
            println!(
                "{}{} -> {}{}",
                piece.as_symbol(),
                move_k.from.as_algebraic().unwrap(),
                piece.as_symbol(),
                move_k.to.as_algebraic().unwrap()
            );
        }
    } else {
        println!("Boom");
    }
}
