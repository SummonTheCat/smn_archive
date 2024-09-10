use std::env;

use crate::tooling::testing::*;

// Command list
const CMD_LIST: [&str; 1] = [
    "test"
];

pub fn run_cmd() {
    let args: Vec<String> = env::args().collect();

    if !validate_cmd(args.clone()) {
        return;
    }

    let cmd = &args[1];

    match cmd.as_str() {
        "test" => {
            cmd_test(args);
        },
        _ => {
            println!("Invalid command: Remove from cmd list!");
        }
    }
}

fn validate_cmd(args: Vec<String>) -> bool {
    if args.len() < 2 {
        println!("Please provide a command");
        return false;
    }

    let cmd = &args[1];

    if !CMD_LIST.contains(&cmd.as_str()) {
        println!("Invalid command: Remove from cmd list!");
        return false;
    }

    return true;
}

// Command functions ---------------------------------------

// Testing
fn cmd_test(args: Vec<String>) {
    println!("Running Tests...");
    println!("{:?}", args);
    test_types();
    test_forms();
}