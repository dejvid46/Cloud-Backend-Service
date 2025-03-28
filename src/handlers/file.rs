use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use std::{env, fs, io::Write, path::Path, path::PathBuf};

use crate::middleware::{CanDownload, CanUpload};
use crate::models::Rename;
use crate::reserr::ResErr;
use crate::utils::dir_size;
use crate::utils::valid_path;

pub async fn get_file_exist(token: CanDownload, req: HttpRequest) -> Result<HttpResponse, ResErr> {
    let path: String = format!(
        "./{}{}/{}",
        env::var("CLOUD_PATH").unwrap(),
        &token.path,
        req.match_info().query("filename")
    );

    if !Path::new(&path).exists() {
        return Err(ResErr::BadClientData("file dont exist"));
    };

    Ok(HttpResponse::Ok().body("file exist"))
}

pub async fn rename_file(
    token: CanUpload,
    req: HttpRequest,
    rename: web::Json<Rename>,
) -> Result<HttpResponse, ResErr> {
    let old_path: String = format!(
        "./{}{}/{}",
        env::var("CLOUD_PATH").unwrap(),
        &token.path,
        req.match_info().query("filename")
    );

    let new_path: String = format!(
        "./{}{}/{}",
        env::var("CLOUD_PATH").unwrap(),
        &token.path,
        rename.name
    );

    fs::rename(old_path, new_path).map_err(|_| ResErr::BadClientData("can not be renamed"))?;

    Ok(HttpResponse::Ok().body("renamed"))
}

pub async fn get_file(token: CanDownload, req: HttpRequest) -> Result<NamedFile, ResErr> {
    let path: String = format!(
        "./{}{}/{}",
        env::var("CLOUD_PATH").unwrap(),
        &token.path,
        req.match_info().query("filename")
    );

    Ok(NamedFile::open(path).map_err(|_| ResErr::BadClientData("file not found"))?)
}

pub async fn post_file(
    token: CanUpload,
    req: HttpRequest,
    mut payload: Multipart,
) -> Result<HttpResponse, ResErr> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = match field.content_disposition() {
            Some(v) => (v),
            None => return Ok(HttpResponse::BadRequest().body("cant find content disposition")),
        };

        let filename = match content_type.get_filename() {
            Some(v) => (v),
            None => return Ok(HttpResponse::BadRequest().body("cant find filename")),
        };

        let main_folder: String = format!("./{}{}", env::var("CLOUD_PATH").unwrap(), &token.path);

        let folder_size: u32 = dir_size(&main_folder)
            .map_err(|_| ResErr::InternalError("folder size counter is broaken"))?;

        if folder_size > token.size {
            return Err(ResErr::BadClientData("you dont have size"));
        }

        let mut filepath: String =
            format!("{}/{}", main_folder, req.match_info().query("filename"));

        valid_path(&filepath).map_err(|err| ResErr::BadClientData(err))?;

        filepath = filepath + "/" + filename;

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .map_err(|_| ResErr::InternalError("field creating file"))?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|_| ResErr::InternalError("field stream of bytes"))?;
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f))
                .await
                .map_err(|_| ResErr::InternalError("field stream of bytes"))?;
        }
    }
    Ok(HttpResponse::Ok().body("file saved"))
}

pub async fn delete_file(token: CanUpload, req: HttpRequest) -> Result<HttpResponse, ResErr> {
    let path: PathBuf =
        (env::var("CLOUD_PATH").unwrap() + &token.path + "/" + req.match_info().query("filename"))
            .parse()
            .map_err(|_| ResErr::BadClientData("cant parse path"))?;

    fs::remove_file(path).map_err(|_| ResErr::BadClientData("file not found"))?;

    Ok(HttpResponse::Ok().body("file deleted"))
}
