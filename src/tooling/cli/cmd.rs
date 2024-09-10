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

    if args.len() < 3 {
        run_test_core();
        return;
    }

    // Pull out the test type we want to run
    let test_type = &args[2];

    match test_type.as_str() {
        "core" => {
            run_test_core();
        },
        "manyforms" => {
            // Pull out the number of forms we want to test
            if args.len() < 4 {
                println!("Please provide the number of forms to test");
                return;
            }

            let num_forms = &args[3];
            let num_forms: u16 = num_forms.parse().unwrap();

            run_test_many_forms(num_forms);
        },
        _ => {
            println!("Invalid test type: {}", test_type);
        }
    }
}

fn run_test_core(){
    test_types();
    test_forms();
    test_io();
}

fn run_test_many_forms(num_forms: u16){
    test_many_forms(num_forms);
}