use std::io::{stdin, BufRead, BufReader, BufWriter, Write};
use std::{env, fs, path, process};

use colored::Colorize;
use walkdir::{DirEntry, WalkDir};

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

    println!(
        "{}",
        "Please select a category by entering the corresponding number:".green()
    );
    let mut c = String::new();
    stdin().read_line(&mut c).unwrap();
    c = c.trim().to_string();
    let category: usize = c.parse::<usize>().unwrap(); // usize bc can't index arrays with i32
    println!(
        "category chosen is {}: {}",
        category,
        categories[category - 1]
    );
    return categories[category - 1].to_string();
}

// Code adapted from https://github.com/BurntSushi/walkdir
fn is_xml(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with("xml"))
        .unwrap_or(false)
}

fn get_file() {
    // Can't use filter_entry bc it's used to filter directories.
    // Use filter map to get rid of Option and then chain filter to get only the xml files.
    for entry in WalkDir::new("./")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_xml(e))
    {
        println!("{}", entry.path().display())
    }
}

fn main() {
    // first check if we're currently in the ../../The Witcher 3/../pc dir
    if !DEBUG && !check_cwd() {
        process::exit(1);
    }
    get_file();
    // get the user's file
    println!("{}", "Enter the name of the file: ".green());
    let user_file = get_user_file();
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
}
