mod board;

use board::{Board, Move};

fn main() {
    let board = Board::default();
    board.print();

    for move_k in Move::generate_moves(&board) {
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

    let fen = "k1K5/8/8/8/3Q4/8/8/8 w KQkq - 0 0";
    str::parse::<Board>(fen).unwrap().print();
}
