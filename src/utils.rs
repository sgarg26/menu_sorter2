/// Convenience functions for working with directories and files.
use std::collections::HashSet;
use walkdir::DirEntry;

// Code adapted from https://github.com/BurntSushi/walkdir
pub fn is_xml(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with("xml"))
        .unwrap_or(false)
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
