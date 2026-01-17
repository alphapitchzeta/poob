use chess::bitboards::*;
use chess::rende::*;

fn main() {
    let board = BitBoards::default();

    println!("{}", board.total_pieces());
    println!("{:b}", board.all_boards());

    print_bitboard(&board);
}
