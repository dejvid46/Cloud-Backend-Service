use std::path::Path;
use validator::ValidationError;
use std::env;

pub fn validate_path(path: &str) -> Result<(), ValidationError> {

    if path.starts_with("./") {
        return Err(ValidationError::new("path cant start with './'"));
    }

    if path.contains("..") {
        return Err(ValidationError::new("you cant use '..' in path"));
    }
    
    if !Path::new(&(env::var("CLOUD_PATH").unwrap()+path)).exists() {
        return Err(ValidationError::new("path dont exist"));
    }

    Ok(())
}

pub fn valid_path(path: &str) -> Result<(), &'static str>  {

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
        if char.is_lowercase(){num_of_lowercase = num_of_lowercase + 1};
        if char.is_uppercase(){num_of_uppercase = num_of_uppercase + 1};
        if char.is_numeric(){num_of_numbers = num_of_numbers + 1};
    }

    if num_of_lowercase > 0 && num_of_uppercase > 0 && num_of_numbers > 0 {
        return Ok(())
    }

    return Err(ValidationError::new("invalid password"));
}