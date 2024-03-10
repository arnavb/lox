use std::{env, process::ExitCode};

fn main() -> ExitCode {
    match env::args().len() {
        1 => run_prompt(),
        2 => run_file(env::args().nth(1).unwrap()),
        _ => {
            eprintln!("Usage: rlox [script]");
            ExitCode::from(64)
        }
    }
}

fn run_file(filename: String) -> ExitCode {
    println!("Running {}", filename);
    ExitCode::SUCCESS
}

fn run_prompt() -> ExitCode {
    println!("Running interpreter");
    ExitCode::SUCCESS
}
