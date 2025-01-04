use std::{
    fs,
    io::{BufRead, BufReader},
    path::PathBuf,
};
use walkdir::{DirEntry, WalkDir};

use rusqlite::{Connection, Result};

fn is_xml(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with("xml"))
        .unwrap_or(false)
}

fn scan_dir(conn: &Connection) {
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

    let files: Vec<PathBuf> = WalkDir::new("./")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_xml(e))
        .map(|e| e.path().to_path_buf())
        .collect();

    // Not great for performance, but only done once when installed.
    // Would be faster with hashset- might do it later.
    for file in files {
        let mut category_found: bool = false;
        let opened = fs::File::open(&file).expect("Unable to open file");
        let reader = BufReader::new(opened);

        let sequence = "Mods.";

        for line in reader.lines() {
            let line = line.expect("Unable to read line");
            for category in categories {
                let cat = format!("{}{}", sequence, category);
                if line.contains(&cat) {
                    println!("{:?} contains {}", file.as_path().strip_prefix(".").unwrap(), category);

                    // If file has already been sorted, add it to the database.
                    let query = format!("INSERT INTO config (file_name, category) VALUES ('{}', '{}')", file.as_path().strip_prefix(".").unwrap().to_str().unwrap(), category);
                    conn.execute(&query, []).unwrap();

                    category_found = true;
                }

                if category_found {
                    break;
                }
            }

            if category_found {
                break;
            }
        }
    }
}

pub fn init_scan(conn: &Connection) {
    scan_dir(conn);
}
