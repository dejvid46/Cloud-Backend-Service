use crate::utils::valid_path;
use actix_web::{Error, HttpRequest, HttpResponse};
use std::env;
use std::fs;
use std::time::SystemTime;

use crate::middleware::{CanDownload, CanUpload};
use crate::models::File;
use crate::reserr::ResErr;
use crate::utils::get_folder_and_files;

pub async fn get_folder(token: CanDownload, req: HttpRequest) -> Result<HttpResponse, ResErr> {
    let path: String =
        (env::var("CLOUD_PATH").unwrap() + &token.path + "/" + req.match_info().query("filename"))
            .parse()
            .map_err(|_| ResErr::BadClientData("cant parse path"))?;

    valid_path(&path).map_err(|err| ResErr::BadClientData(err))?;

    let paths = fs::read_dir(path).map_err(|_| ResErr::BadClientData("cant find path"))?;

    let mut res: Vec<File> = Vec::new();
    let now = SystemTime::now();

    for path in paths {
        let better_path = path
            .map_err(|_| ResErr::BadClientData("cant do path"))?
            .path();
        let metadata =
            fs::metadata(&better_path).map_err(|_| ResErr::BadClientData("cant get metadata"))?;
        res.push(File {
            name: better_path
                .to_str()
                .unwrap()
                .replace("\\", "/")
                .split('/')
                .last()
                .unwrap()
                .to_string(),
            date: match metadata.modified() {
                Ok(v) => match now.duration_since(v) {
                    Ok(v) => v.as_secs().to_string(),
                    Err(_) => "time is broken".to_string(),
                },
                Err(_) => "no supported".to_string(),
            },
            size: metadata.len(),
        });
    }

    Ok(HttpResponse::Ok().json(res))
}

pub async fn create_folder(token: CanUpload, req: HttpRequest) -> Result<HttpResponse, ResErr> {
    let path: String =
        (env::var("CLOUD_PATH").unwrap() + &token.path + "/" + req.match_info().query("filename"))
            .parse()
            .map_err(|_| ResErr::BadClientData("cant parse path"))?;

    fs::create_dir_all(path).map_err(|_| ResErr::BadClientData("cant create folder"))?;

    Ok(HttpResponse::Ok().body("folder created"))
}

pub async fn delete_folder(token: CanUpload, req: HttpRequest) -> Result<HttpResponse, Error> {
    let path: String =
        (env::var("CLOUD_PATH").unwrap() + &token.path + "/" + req.match_info().query("filename"))
            .parse()?;

    valid_path(&path).map_err(|err| ResErr::BadClientData(err))?;

    fs::remove_dir_all(path).map_err(|_| ResErr::BadClientData("cant remove folder"))?;
    Ok(HttpResponse::Ok().body("folder deleted"))
}

pub async fn get_tree(token: CanDownload, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let main_folder: String = format!("./{}{}", env::var("CLOUD_PATH").unwrap(), &token.path);

    valid_path(&main_folder).map_err(|err| ResErr::BadClientData(err))?;

    Ok(HttpResponse::Ok().json(get_folder_and_files(main_folder)))
}
