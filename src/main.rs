//use poob::bitboards::*;
//use poob::boardstate::*;
//use poob::rende::*;
//use poob::game::Game;
//use poob::moves::*;
use poob::cli::Session;
use poob::perft::*;

use std::fs::{self, File};
use std::io::BufRead;
use std::io::{self, BufReader, Write};

fn main() {
    let mut session = Session::new();

    session.run();

    //let path = read_perft();

    //perft_suite(&path);
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
    let test_case_file = fs::File::open(path).expect("Unable to read file");

    let mut out_file = File::create_new("perft_results.txt").expect("Unable to create file");

    let (mut successes, mut failures) = (0, 0);

    for (i, line) in BufReader::new(test_case_file).lines().enumerate() {
        let line = match line {
            Ok(s) => s,
            Err(_) => break,
        };

        //eprintln!("Line: {line}");

        let test_case = match PerftCase::from_str(line.trim()) {
            Some(case) => case,
            None => {
                eprintln!("Something failed here");
                break;
            }
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
