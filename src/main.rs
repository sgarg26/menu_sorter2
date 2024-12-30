use std::io::stdin;
use std::{env, process};

fn check_cwd() -> bool {
    let cwd = env::current_dir().unwrap().to_path_buf();
    if !cwd.ends_with("user_config_matrix/pc") {
        println!("Please run this script from the ../../user_config_matrix/pc directory");
        return false;
    }
    true
}

fn main() {
    // first check if we're currently in the ../../The Witcher 3/../pc dir
    if !check_cwd() {
        process::exit(1);
    }
}
