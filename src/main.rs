use std::io::{BufReader, BufRead, stdin};
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

fn main() {
    // first check if we're currently in the ../../The Witcher 3/../pc dir
    if !DEBUG && !check_cwd() {
        process::exit(1);
    }

    // get the user's file
    let user_file = get_user_file();
    println!("User file: {:?}", user_file);
    
    let path = path::Path::new(&user_file);
    
    // Try opening the file, panic if fails.
    let mut file = match fs::File::open(&path) {
        Err(why) => panic!("Couldn't open {:?}: {}", path, why),
        Ok(file) => file
    };
    
    println!("File opened successfully");
    
    // Read the file into a buffer to edit.
    // Using BufReader, more efficient when reading line by line.
    // Better in this case, bc end goal is to see if a certain line contains 'Mods...'
    let file = BufReader::new(file);
    
    for line in file.lines() {
        let line = line.expect("Unable to read line");
        println!("{}", line);
    }
    
}
