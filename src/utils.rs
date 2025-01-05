/// Convenience functions

use std::env;
use std::collections::HashSet;
use walkdir::DirEntry;



pub fn is_xml(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with("xml"))
        .unwrap_or(false)
}

pub fn check_cwd() -> bool {
    let cwd = env::current_dir().unwrap().to_path_buf();
    if !cwd.ends_with("user_config_matrix/pc") {
        println!("Please run this script from the ../../user_config_matrix/pc directory");
        return false;
    }
    true
}

pub fn is_settings_file(entry: &DirEntry) -> bool {
    // the settings files provided by CDPR are probably not want users want to place in other directories.
    let settings_files = HashSet::from([
        "audio.xml",
        "display.xml",
        "gameplay.xml",
        "gamma.xml",
        "graphics.xml",
        "graphicsdx11.xml",
        "hud.xml",
        "hidden.xml",
        "hdr.xml",
        "input.xml",
    ]);

    entry
        .file_name()
        .to_str()
        .map(|s| settings_files.contains(s))
        .unwrap_or(false)
}