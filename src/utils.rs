use std::convert::TryFrom;
use std::env;
use std::path::Path;
use std::{fs, io, path::PathBuf};
use validator::ValidationError;

use crate::models::Folder;

pub fn validate_path(path: &str) -> Result<(), ValidationError> {
    if path.starts_with("./") {
        return Err(ValidationError::new("path cant start with './'"));
    }

    if path.contains("..") {
        return Err(ValidationError::new("you cant use '..' in path"));
    }

    if !Path::new(&(env::var("CLOUD_PATH").unwrap() + path)).exists() {
        return Err(ValidationError::new("path dont exist"));
    }

    Ok(())
}

pub fn valid_path(path: &str) -> Result<(), &'static str> {
    if path.contains("..") {
        return Err("you cant use '..' in path");
    }

    if !Path::new(path).exists() {
        return Err("path dont exist");
    }

    Ok(())
}

pub fn valid_pass(pass: &str) -> Result<(), ValidationError> {
    let mut num_of_lowercase = 0;
    let mut num_of_uppercase = 0;
    let mut num_of_numbers = 0;

    for char in pass.chars() {
        if char.is_lowercase() {
            num_of_lowercase = num_of_lowercase + 1
        };
        if char.is_uppercase() {
            num_of_uppercase = num_of_uppercase + 1
        };
        if char.is_numeric() {
            num_of_numbers = num_of_numbers + 1
        };
    }

    if num_of_lowercase <= 0 {
        return Err(ValidationError::new("pass must include lowercase"));
    }

    if num_of_uppercase <= 0 {
        return Err(ValidationError::new("pass must include uppercase"));
    }

    if num_of_numbers <= 0 {
        return Err(ValidationError::new("pass must include number"));
    }
    Ok(())
}

pub fn dir_size(path: impl Into<PathBuf>) -> io::Result<u32> {
    fn dir_size(mut dir: fs::ReadDir) -> io::Result<u32> {
        dir.try_fold(0, |acc, file| {
            let file = file?;
            let size = match file.metadata()? {
                data if data.is_dir() => dir_size(fs::read_dir(file.path())?)?,
                data => u64_to_u32(data.len()),
            };
            Ok(acc + size)
        })
    }

    dir_size(fs::read_dir(path.into())?)
}

fn u64_to_u32(v: u64) -> u32 {
    u32::try_from(v / 1000000).unwrap_or_default()
}

fn get_folder_obj(path: String, obj: &mut Folder) {
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.

                let file_name = entry.file_name().into_string().unwrap();

                if file_name.contains(".") {
                    obj.folders.push(Folder {
                        name: file_name,
                        folders: Vec::with_capacity(0),
                    });
                } else {
                    obj.folders.push(Folder {
                        name: file_name.clone(),
                        folders: Vec::with_capacity(0),
                    });
                    get_folder_obj(
                        format!("{}{}{}", path.clone(), "/", file_name),
                        &mut obj.folders.last_mut().unwrap(),
                    );
                }
            }
        }
    }
}

pub fn get_folder_and_files(path: String) -> Folder {
    let mut main_folder = Folder {
        name: String::from(""),
        folders: Vec::new(),
    };

    get_folder_obj(path, &mut main_folder);

    return main_folder;
}
