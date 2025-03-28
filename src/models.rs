use crate::utils::{valid_pass, validate_path};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct User {
    pub id: u32,
    #[validate(length(min = 4, max = 20, code = "name min 4 max 20 letters"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, code = "pass min 8 letters"), custom = "valid_pass")]
    pub pass: String,
    pub size: u32,
    #[validate(custom = "validate_path")]
    pub path: String,
    pub status: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rename {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct ChangingUser {
    #[validate(length(min = 4, max = 20, code = "name min 4 max 20 letters"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, code = "pass min 8 letters"), custom = "valid_pass")]
    pub pass: String,
}

pub enum Queries {
    GetUserById(u32),
    GetAllUsers,
    DeleteUserById(u32),
    AddUser(Json<User>),
    UpdateUserById(u32, Json<User>),
    UpdateMeById(u32, Json<ChangingUser>),
    GetUserByEmail(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub name: String,
    pub date: String,
    pub size: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub id: u32,
    pub exp: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Login {
    pub email: String,
    pub pass: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Folder {
    pub name: String,
    pub folders: Vec<Folder>,
}
