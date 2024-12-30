use std::io::stdin;
use std::{env, process};

fn check_cwd() -> bool {
    let cwd = env::current_dir().unwrap().to_path_buf();
    if !cwd.ends_with("The Witcher 3") {
        println!("Please run this script from the ../../The Witcher 3 directory");
        return false;
    }
    true
}

fn main() {
    // first check if we're currently in the ../../The Witcher 3 directory
    if !check_cwd() {
        process::exit(1);
    }
}
