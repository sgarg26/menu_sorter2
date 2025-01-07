use path::PathBuf;
use std::io::{stdin, BufRead, BufReader, BufWriter, Write};
use std::{env, fs, path, process};

use colored::Colorize;
use configparser::ini::Ini;
use walkdir::WalkDir;

use menu_sorter::init_scan;

mod utils;
use utils::{is_settings_file, is_xml};

const DEBUG: bool = true;

fn check_cwd() -> bool {
    let cwd = env::current_dir().unwrap().to_path_buf();
    if !cwd.ends_with("user_config_matrix/pc") {
        println!("Please run this script from the ../../user_config_matrix/pc directory");
        return false;
    }
    true
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
        println!("{}: {}", count, file.strip_prefix("./").unwrap().display());
        count += 1;
    }

    loop {
        if files.is_empty() {
            println!("No files found. Exiting.");
            process::exit(0);
        }
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

fn main() {
    // For debug purposes. Ignore.
    if DEBUG {
        // env::set_current_dir("./src").expect("Something went wrong");
    }
    let mut config = Ini::new();
    // If first time using the app, scan the directory, and
    // add all currently existing files and their categories
    // to the config file.
    if !path::Path::new("./menu_sorter.ini").exists() {
        fs::File::create("./menu_sorter.ini").unwrap();
        fs::write("./menu_sorter.ini", "[files]").expect("Unable to write");
        init_scan(&mut config);
    }

    let map = config.load("menu_sorter.ini").unwrap();
    let file_list = map.get("files").unwrap();

    // first check if we're currently in the ../../The Witcher 3/../pc dir
    if !DEBUG && !check_cwd() {
        process::exit(1);
    }

    // bool to check if the file contains a category already
    // if it does, that needs to be replaced w the new category (or not)
    let mut file_contains_category = false;
    let mut old_category = String::new();

    // get the user's file
    let user_file = get_file();
    let trimmed_user_file = user_file.strip_prefix("./").unwrap().to_str().unwrap();
    let trimmed_user_file = &trimmed_user_file.to_string().to_lowercase();
    println!("User file: {:?}", trimmed_user_file);
    if file_list.contains_key(trimmed_user_file) {
        old_category = file_list
            .get(trimmed_user_file)
            .unwrap()
            .to_owned()
            .unwrap(); // kill me
        println!(
            "{:?} with category {} found.",
            trimmed_user_file, old_category
        );
        file_contains_category = true;
    }

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
    let rep_with = format!("{}{}.", sequence, category);
    let mut rep_from = String::new();
    if file_contains_category {
        rep_from = format!("{}{}.", sequence, old_category);
        // if the old and new category are the same, no changes needed.
        if rep_from == rep_with {
            println!("Category already exists. No changes needed. Exiting.");
            process::exit(0);
        }
        // otherwise, updated the config file.
        else {
            config.set("files", trimmed_user_file, Some(category));
            config
                .write("menu_sorter.ini")
                .expect("Unable to write to file");
        }
    }

    // We'll write to a temp file, then overwrite the original file with the temp file.
    let tmp_path = format!("{}.tmp", path.display());
    let tmp_file = fs::File::create(tmp_path).expect("Unable to create file");

    // do the same thing as above, but BufWriter instead of BufReader
    let mut tmp_file = BufWriter::new(tmp_file);

    for line in file.lines() {
        let line = line.expect("Unable to read line");
        // let mut new_line = line.replace(sequence, &rep_with);
        let mut new_line = String::new();
        if file_contains_category {
            new_line = line.replace(&rep_from, &rep_with);
        } else {
            new_line = line.replace(sequence, &rep_with);
        }
        new_line.push('\n'); // new line char gets removed for some reason? not sure why.
        tmp_file
            .write_all(new_line.as_bytes())
            .expect("Unable to write data")
    }
    // Rename tmp file to permanent file later.
    fs::rename(format!("{}.tmp", path.display()), path).expect("Unable to rename temp file");
    println!("All done, exiting cleanly.");
}
