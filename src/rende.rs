use crate::Color;
use crate::bitboard_types::*;
use crate::bitboards::BitBoards;

pub fn print_bitboard(bitboard: &BitBoards, perspective: Color) {
    let mut board_arr = [[' '; 8]; 8];

    set_pieces(&mut board_arr, bitboard.pawns_white(), 'P');
    set_pieces(&mut board_arr, bitboard.pawns_black(), 'p');
    set_pieces(&mut board_arr, bitboard.rooks_white(), 'R');
    set_pieces(&mut board_arr, bitboard.rooks_black(), 'r');
    set_pieces(&mut board_arr, bitboard.knights_white(), 'N');
    set_pieces(&mut board_arr, bitboard.knights_black(), 'n');
    set_pieces(&mut board_arr, bitboard.bishops_white(), 'B');
    set_pieces(&mut board_arr, bitboard.bishops_black(), 'b');
    set_pieces(&mut board_arr, bitboard.queens_white(), 'Q');
    set_pieces(&mut board_arr, bitboard.queens_black(), 'q');
    set_pieces(&mut board_arr, bitboard.king_white(), 'K');
    set_pieces(&mut board_arr, bitboard.king_black(), 'k');

    match perspective {
        Color::White => {
            for (rank, name) in board_arr.iter().rev().zip((1..=8).rev()) {
                for square in rank.iter() {
                    print!("[{}]", square);
                }

                print!(" {name}");

                print!("\n");
            }
        }
        Color::Black => {
            for (rank, name) in board_arr.iter().zip(1..=8) {
                for square in rank.iter() {
                    print!("[{}]", square);
                }

                print!(" {name}");

                print!("\n");
            }
        }
    };

    //print!("  ");

    for c in 'a'..='h' {
        print!(" {c} ");
    }

    print!("\n");
}

fn set_pieces(board: &mut [[char; 8]; 8], bitboard: BitBoard, c: char) {
    for i in 0..64 {
        if bitboard & BitBoard(1 << i) == BitBoard(0) {
            continue;
        }

        let (x, y) = bit_to_2d_indices(i);

        board[x][y] = c;
    }
}

fn bit_to_2d_indices(bit: u64) -> (usize, usize) {
    ((bit / 8) as usize, (bit % 8) as usize)
}
