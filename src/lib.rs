use configparser::ini::{Ini, WriteOptions};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
use walkdir::WalkDir;

mod utils;
use utils::{is_settings_file, is_xml};

fn get_category_from_file(entry: &File) -> Option<String> {
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
    let mut ret_category = String::new();
    let reader = BufReader::new(entry);

    for line in reader.lines() {
        let line = line.unwrap();
        for category in categories {
            let cat = format!("{}{}", "Mods.", category);
            if line.contains(cat.as_str()) {
                ret_category = category.to_string();
                return Some(ret_category);
            }
        }
    }

    None
}

pub fn init_scan(config: &mut Ini) {
    let mut valid_files_found = false;
    let files: Vec<PathBuf> = WalkDir::new("./")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_xml(e) && !is_settings_file(e))
        .map(|e| e.path().to_path_buf())
        .collect();

    for file in files {
        let category = get_category_from_file(&File::open(&file).unwrap());
        valid_files_found = true;
        if category.is_some() {
            config.set(
                "files",
                file.strip_prefix(".").unwrap().to_str().unwrap(),
                category,
            );
        }
    }
    let write_options = WriteOptions::new_with_params(true, 2, 1);
    if valid_files_found {
        config
            .pretty_write("menu_sorter.ini", &write_options)
            .unwrap()
    };
}
