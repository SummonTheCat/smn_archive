// -- Core Library Modules -- 
mod core;
mod tooling;

fn main() {
    /*
    Run the cmd tools for testing and utility
    E.g. cargo run -- tool arg1 arg2
    */
    tooling::cli::run_cmd();
}   