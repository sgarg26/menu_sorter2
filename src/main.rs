use std::io::stdin;
use std::{env, fs, path, process};

const DEBUG: bool = true;

fn check_cwd() -> bool {
    let cwd = env::current_dir().unwrap().to_path_buf();
    if !cwd.ends_with("user_config_matrix/pc") {
        println!("Please run this script from the ../../user_config_matrix/pc directory");
        return false;
    }
    true
}

fn get_user_file() -> path::PathBuf {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer = buffer.trim().to_string();
    let path = path::Path::new(&buffer);
    path.to_path_buf()
}

fn check_exists(file: path::PathBuf) -> bool {
    if !file.exists() {
        println!("File {:?} does not exist!", file);
        return false;
    }
    true
}

fn main() {
    // first check if we're currently in the ../../The Witcher 3/../pc dir
    if !DEBUG && !check_cwd() {
        process::exit(1);
    }

    // get the user's file
    let user_file = get_user_file();
    println!("User file: {:?}", user_file);
    
    // check if the file exists
    if !check_exists(user_file) {
        process::exit(1);
    }
    
    println!("Hello")
}
