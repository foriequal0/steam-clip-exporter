use crate::SteamRoot;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LibraryCacheError {
    #[error("File not found")]
    CannotOpenAcf,
    #[error("name not found in acf file")]
    NameNotFound,
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn try_get_app_name(steam_root: &SteamRoot, app_id: &str) -> Result<String, LibraryCacheError> {
    let regex = Regex::new(r#"^\s*"name"\s*"(?<name>.*?)"\s*$"#).expect("Invalid regex");

    for library_folder in get_library_folders(steam_root) {
        let acf_file = library_folder.join(format!("steamapps/appmanifest_{}.acf", app_id));
        let Ok(file) = File::open(acf_file) else {
            continue;
        };

        let file = BufReader::new(file);
        for line in file.lines() {
            let line = line?;
            if let Some(captures) = regex.captures(&line) {
                return Ok(captures.name("name").expect("captured").as_str().to_owned());
            }
        }
    }

    Err(LibraryCacheError::NameNotFound)
}

fn get_library_folders(steam_root: &SteamRoot) -> Vec<PathBuf> {
    let mut result = Vec::new();
    result.push(steam_root.root().to_owned());

    let libraryfolders = steam_root.join("steamapps/libraryfolders.vdf");
    _ = parse_library_folders(&mut result, &libraryfolders);

    return result;

    fn parse_library_folders(
        result: &mut Vec<PathBuf>,
        path: &Path,
    ) -> Result<(), LibraryCacheError> {
        let file = BufReader::new(File::open(path).map_err(|_| LibraryCacheError::CannotOpenAcf)?);

        let regex = Regex::new(r#"^\s*"path"\s*"(?<path>.*?)"\s*$"#).expect("Invalid regex");
        for line in file.lines() {
            let line = line?;
            if let Some(captures) = regex.captures(&line) {
                let path = captures.name("path").expect("captured").as_str();
                result.push(PathBuf::from(path));
            }
        }

        Ok(())
    }
}

#[allow(unused)]
pub struct GameMedia {
    // logo, hero, header, icon
}

#[allow(unused)]
pub fn get_media(_steam_root: &Path, _app_id: String) -> Result<String, LibraryCacheError> {
    // $STEAM_ROOT/appcache/librarycache/${APP_ID}/**/*.{jpg,png}

    unimplemented!()
}
