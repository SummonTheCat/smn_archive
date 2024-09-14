use std::env;

use crate::tooling::{automation, testing::*};

// Command list
const CMD_LIST: [&str; 2] = [
    "test",
    "gen",
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
        "gen" => {
            cmn_gen(args);
        },
        _ => {
            println!("Invalid command, available commands are:");
            for cmd in CMD_LIST.iter() {
                println!("-> {}", cmd);
            }
        }
    }
}

fn validate_cmd(args: Vec<String>) -> bool {
    if args.len() < 2 {
        println!("Please provide a command");
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

    // Extract the test type from the arguments
    let test_type = &args[2];

    match test_type.as_str() {
        "core" => {
            run_test_core();
        },
        "manyformsthreaded" => {
            // Check if we have enough arguments
            if args.len() < 6 {
                println!("Usage: test manyformsthreaded [r/w/rw/wr] [Form Count] [Thread Count]");
                return;
            }

            // Extract the operation type (r, w, rw, wr)
            let operation_type = &args[3];

            // Extract the number of forms to test
            let num_forms_str = &args[4];
            let num_forms: u16 = match num_forms_str.parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid number of forms: {}", num_forms_str);
                    return;
                }
            };

            // Extract the number of threads to use
            let num_threads_str = &args[5];
            let num_threads: usize = match num_threads_str.parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid number of threads: {}", num_threads_str);
                    return;
                }
            };

            run_test_many_forms_threaded(operation_type, num_forms, num_threads);
        },
        _ => {
            println!("Invalid test type: {}", test_type);
        }
    }
}

fn run_test_many_forms_threaded(operation_type: &str, num_forms: u16, num_threads: usize) {
    match operation_type {
        "r" => {
            test_read_forms_many_threaded(num_forms, num_threads)
        },
        "w" => {
            test_write_forms_many_threaded(num_forms)
        }, 
        "rw" => {
            test_write_forms_many_threaded(num_forms);
            test_read_forms_many_threaded(num_forms, num_threads);
        },
        "wr" => {
            test_write_forms_many_threaded(num_forms);
            test_read_forms_many_threaded(num_forms, num_threads);
        },
        _ => {
            println!("Invalid operation type: {}", operation_type);
        }
        
    }
}
fn run_test_core(){
    test_types();
    test_forms();
    test_io();
}



// Formtype generation ---------------------------------------
fn cmn_gen(args: Vec<String>) {
    if args.len() < 3 {
        println!("Please state the type of generation you want to perform");
        return;
    }

    let gen_type = &args[2];

    match gen_type.as_str() {
        "formtype" => {
            if args.len() < 4 {
                println!("Please provide the name of the formtype you want to generate");
                return;
            }
            let formtype_name = &args[3];
            let _ = automation::formtype_management::formtype_add(formtype_name);
        },
        _ => {
            println!("Invalid generation type: {}", gen_type);
        }
    }






    

    
}