use actix_web::{web, dev, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};

use crate::db::{execute, Pool};
use crate::jwt::authorize;
use crate::models::{Queries, User};
use crate::reserr::ResErr;

pub struct MustLogin {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub pass: String,
    pub size: u32,
    pub path: String,
    pub status: u8
}

pub struct MustAdmin {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub pass: String,
    pub size: u32,
    pub path: String,
    pub status: u8
}

pub struct MustAdminOrOp {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub pass: String,
    pub size: u32,
    pub path: String,
    pub status: u8
}

pub struct CanUpload {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub pass: String,
    pub size: u32,
    pub path: String,
    pub status: u8
}

pub struct CanDownload {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub pass: String,
    pub size: u32,
    pub path: String,
    pub status: u8
}

macro_rules! impl_T {
    (for $($t:ident),+) => {
        $(impl From<User> for $t {
            fn from(user: User) -> Self {
                let User { id, name, email, pass, size, path, status } = user;
                $t {
                    id,
                    name,
                    email,
                    pass,
                    size,
                    path,
                    status
                }
            }
        })*
    }
}

impl_T!(for MustLogin, MustAdmin, MustAdminOrOp, CanUpload, CanDownload);

impl FromRequest for MustLogin {
    type Error = ResErr;
    type Future = Ready<Result<MustLogin, ResErr>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        if let Some(db) = req.app_data::<web::Data<Pool>>() {
            match authorize(&req) {
                Ok(token) => {
                    let res = match execute(&db, Queries::GetUserById(token.id)){
                        Ok(v) => v.one(),
                        Err(_) => return err(ResErr::BadClientData("cant get user"))
                    };
                    ok(MustLogin::from(res))
                },
                Err(error) => err(ResErr::BadClientData(error))
            }
        }else{
            err(ResErr::BadClientData("cant use db"))
        }
    }
}

//pub struct MustAdmin;

impl FromRequest for MustAdmin {
    type Error = ResErr;
    type Future = Ready<Result<MustAdmin, ResErr>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {

        if let Some(db) = req.app_data::<web::Data<Pool>>() {
            match authorize(&req) {
                Ok(token) => {
                    let res = match execute(&db, Queries::GetUserById(token.id)){
                        Ok(v) => v.one(),
                        Err(_) => return err(ResErr::BadClientData("cant get user"))
                    };

                    if res.status == 1 {
                        ok(MustAdmin::from(res))
                    }else{
                        err(ResErr::BadClientData("must be Admin"))
                    }
                },
                Err(error) => err(ResErr::BadClientData(error))
            }
        }else{
            err(ResErr::BadClientData("cant use db"))
        }
    }
}

impl FromRequest for MustAdminOrOp {
    type Error = ResErr;
    type Future = Ready<Result<MustAdminOrOp, ResErr>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {

        if let Some(db) = req.app_data::<web::Data<Pool>>() {
            match authorize(&req) {
                Ok(token) => {
                    let res = match execute(&db, Queries::GetUserById(token.id)){
                        Ok(v) => v.one(),
                        Err(_) => return err(ResErr::BadClientData("cant get user"))
                    };

                    if res.status == 1 || res.status == 2 {
                        ok(MustAdminOrOp::from(res))
                    }else{
                        err(ResErr::BadClientData("must be Admin or Op"))
                    }
                },
                Err(error) => err(ResErr::BadClientData(error))
            }
        }else{
            err(ResErr::BadClientData("cant use db"))
        }
    }
}

impl FromRequest for CanUpload {
    type Error = ResErr;
    type Future = Ready<Result<CanUpload, ResErr>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {

        if let Some(db) = req.app_data::<web::Data<Pool>>() {
            match authorize(&req) {
                Ok(token) => {
                    let res = match execute(&db, Queries::GetUserById(token.id)){
                        Ok(v) => v.one(),
                        Err(_) => return err(ResErr::BadClientData("cant get user"))
                    };

                    if res.status == 1 || res.status == 2 || res.status == 3 {
                        ok(CanUpload::from(res))
                    }else{
                        err(ResErr::BadClientData("cant upload"))
                    }
                },
                Err(error) => err(ResErr::BadClientData(error))
            }
        }else{
            err(ResErr::BadClientData("cant use db"))
        }
    }
}

impl FromRequest for CanDownload {
    type Error = ResErr;
    type Future = Ready<Result<CanDownload, ResErr>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {

        if let Some(db) = req.app_data::<web::Data<Pool>>() {
            match authorize(&req) {
                Ok(token) => {
                    let res = match execute(&db, Queries::GetUserById(token.id)){
                        Ok(v) => v.one(),
                        Err(_) => return err(ResErr::BadClientData("cant get user"))
                    };

                    if res.status == 1 || res.status == 2 || res.status == 3 || res.status == 4 {
                        ok(CanDownload::from(res))
                    }else{
                        err(ResErr::BadClientData("cant download"))
                    }
                },
                Err(error) => err(ResErr::BadClientData(error))
            }
        }else{
            err(ResErr::BadClientData("cant use db"))
        }
    }
}