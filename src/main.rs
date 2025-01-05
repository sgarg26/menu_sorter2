use path::PathBuf;
use std::collections::HashSet;
use std::io::{stdin, BufRead, BufReader, BufWriter, Write};
use std::{env, fs, path, process};

use colored::Colorize;
use rusqlite::{Connection, Result};
use walkdir::{DirEntry, WalkDir};

use menu_sorter::init_scan;

mod utils;
use utils::*;

const DEBUG: bool = true;

#[derive(Debug)]
struct FileCategory {
    file_name: String,
    category: String,
}



fn get_category() -> String {
    let categories = [
        "alchemy_and_equipment",
        "camera",
        "characters",
        "combat",
        "gameplay",
        "quests_and_adventures",
        "user_interface",
        "visuals_and_graphics",
        "miscellaneous",
    ];
    let mut count = 1;
    for category in categories {
        println!("{}. {}", count, category);
        count += 1;
    }

    loop {
        println!(
            "{}",
            "Please select a category by entering the corresponding number:".green()
        );
        let mut c = String::new();
        stdin().read_line(&mut c).unwrap();
        c = c.trim().to_string();
        let category: usize = match c.parse::<usize>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if category > 0 && category <= categories.len() {
            return categories[category - 1].to_string();
        }
    }
}




// Code adapted from https://github.com/BurntSushi/walkdir
fn get_file() -> PathBuf {
    let mut count = 1;
    // Can't use filter_entry bc it's used to filter directories.
    // Use filter map to get rid of Option and then chain filter to get only the xml files.
    let files: Vec<PathBuf> = WalkDir::new("./")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_xml(e) && !is_settings_file(e))
        .map(|e| e.path().to_path_buf())
        .collect();

    for file in &files {
        println!("{}: {}", count, file.display());
        count += 1;
    }

    loop {
        let mut c = String::new();
        println!(
            "{} {}",
            "Enter file index between 1 and".green(),
            files.len().to_string().green()
        );
        stdin().read_line(&mut c).unwrap();
        c = c.trim().to_string();
        let file = match c.parse::<usize>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if file > 0 && file <= files.len() {
            return files[file - 1].clone();
        }
    }
}

fn open_db() -> Connection {
    let conn = match Connection::open("config.db3") {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error opening database: {:?}", e);
            process::exit(1);
        }
    };

    let query = "
    CREATE TABLE IF NOT EXISTS config (
        file_name VARCHAR PRIMARY KEY,
        category VARCHAR NOT NULL
    )
    ";

    conn.execute(query, []).expect("Unable to create table");

    conn
}

fn main() {
    let scan_needed = if path::Path::new("config.db3").exists() {
        false
    } else {
        true
    };
    let conn = open_db();

    if scan_needed {
        init_scan(&conn);
    }

    // first check if we're currently in the ../../The Witcher 3/../pc dir
    if !DEBUG && !check_cwd() {
        process::exit(1);
    }

    // get the user's file
    let user_file = get_file();
    println!("User file: {:?}", user_file);

    let path = path::Path::new(&user_file);

    // Try opening the file, panic if fails.
    let file = match fs::File::open(&path) {
        Err(why) => panic!("Couldn't open {:?}: {}", path, why),
        Ok(file) => file,
    };

    println!("File opened successfully");
    // Get the category the user wants to add to.
    let category = get_category();

    // Read the file into a buffer to edit.
    // Using BufReader, more efficient when reading line by line.
    // Better in this case, bc end goal is to see if a certain line contains 'Mods...'
    let file = BufReader::new(file);

    // When adding to a category, category is always added right after 'Mods.'
    // The sequence is unique, so we can use it to find the line we want to edit.
    let sequence = "Mods.";
    let rep = format!("{}{}.", sequence, category);

    // We'll write to a temp file, then overwrite the original file with the temp file.
    let tmp_path = format!("{}.tmp", path.display());
    let tmp_file = fs::File::create(tmp_path).expect("Unable to create file");

    // do the same thing as above, but BufWriter instead of BufReader
    let mut tmp_file = BufWriter::new(tmp_file);

    for line in file.lines() {
        let line = line.expect("Unable to read line");
        let mut new_line = line.replace(sequence, &rep);
        new_line.push('\n'); // new line char gets removed for some reason? not sure why.
        tmp_file
            .write_all(new_line.as_bytes())
            .expect("Unable to write data")
    }
    // Rename tmp file to permanent file later.
    fs::rename(format!("{}.tmp", path.display()), path).expect("Unable to rename temp file");
}
