use crate::bitboards::BitBoards;
use crate::boardstate::Color;

pub fn print_bitboard(bitboard: &BitBoards, perspective: Color) {
    let mut board_arr = [[' '; 8]; 8];

    set_pieces(&mut board_arr, bitboard.pawns_white(), '♟');
    set_pieces(&mut board_arr, bitboard.pawns_black(), '♙');
    set_pieces(&mut board_arr, bitboard.rooks_white(), '♜');
    set_pieces(&mut board_arr, bitboard.rooks_black(), '♖');
    set_pieces(&mut board_arr, bitboard.knights_white(), '♞');
    set_pieces(&mut board_arr, bitboard.knights_black(), '♘');
    set_pieces(&mut board_arr, bitboard.bishops_white(), '♝');
    set_pieces(&mut board_arr, bitboard.bishops_black(), '♗');
    set_pieces(&mut board_arr, bitboard.queens_white(), '♛');
    set_pieces(&mut board_arr, bitboard.queens_black(), '♕');
    set_pieces(&mut board_arr, bitboard.king_white(), '♚');
    set_pieces(&mut board_arr, bitboard.king_black(), '♔');

    print!("  ");

    for c in 'a'..='h' {
        print!(" {c} ");
    }

    print!("\n");

    match perspective {
        Color::White => {
            for (rank, name) in board_arr.iter().rev().zip((1..=8).rev()) {
                print!("{name} ");

                for square in rank.iter().rev() {
                    print!("[{}]", square);
                }

                print!("\n");
            }
        }
        Color::Black => {
            for (rank, name) in board_arr.iter().zip(1..=8) {
                print!("{name} ");

                for square in rank.iter().rev() {
                    print!("[{}]", square);
                }

                print!("\n");
            }
        }
    };
}

fn set_pieces(board: &mut [[char; 8]; 8], bitboard: u64, c: char) {
    for i in 0..64 {
        if bitboard & 1 << i == 0 {
            continue;
        }

        let (x, y) = bit_to_2d_indices(i);

        board[x][y] = c;
    }
}

fn bit_to_2d_indices(bit: u64) -> (usize, usize) {
    ((bit / 8) as usize, (bit % 8) as usize)
}
