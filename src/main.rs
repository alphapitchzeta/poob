//use chess::bitboards::*;
//use chess::boardstate::*;
//use chess::rende::*;
use chess::game::Game;
//use chess::moves::*;
use chess::movegen::*;
use chess::perft::*;

use std::io::Write;
use std::{
    fs::{self, File},
    io,
};

fn main() {

}

#[allow(dead_code)]
fn read_perft() -> String {
    let mut buf = String::new();

    print!("Perft suite file path: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");

    buf.trim().to_string()
}

#[allow(dead_code)]
fn perft_suite(path: &str) {
    let test_cases = fs::read_to_string(path).expect("Unable to read file");

    let mut out_file = File::create_new("perft_results.txt").expect("Unable to create file");

    let move_gen = MoveGenerator::new();

    let (mut successes, mut failures) = (0, 0);

    for (i, line) in test_cases.split('\n').enumerate() {
        let test_case = match PerftCase::from_str(line, &move_gen) {
            Some(case) => case,
            None => break,
        };

        println!("Testing case {}...", i + 1);
        write!(&mut out_file, "Test Case {}\n", i + 1).expect("Failed to write to file");

        let (results, success) = test_case.test();

        if success {
            successes += 1;
        } else {
            failures += 1;
        }

        write!(&mut out_file, "{results}---\n\n").expect("Failed to write to file");
    }

    write!(
        &mut out_file,
        "Successfully tested positions: {successes}\nFailures: {failures}"
    )
    .expect("Failed to write to file");
}
