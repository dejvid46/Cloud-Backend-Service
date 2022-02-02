use actix_web::{web, HttpResponse};
use bcrypt::{hash, DEFAULT_COST};
use validator::Validate;

use crate::db::{execute, Pool};
use crate::middleware::MustAdminOrOp;
use crate::models::{Queries, User};
use crate::reserr::ResErr;

pub async fn get_users(_: MustAdminOrOp, db: web::Data<Pool>) -> Result<HttpResponse, ResErr> {
    let mut user = (execute(&db, Queries::GetAllUsers)
        .map_err(|_| ResErr::BadClientData("cant get users"))?)
    .many();

    let mut x = user.len() - 1;
    while x > 0 {
        user[x].pass = "".to_string();
        x -= 1;
    }

    Ok(HttpResponse::Ok().json(user))
}

pub async fn get_user_by_id(
    _: MustAdminOrOp,
    db: web::Data<Pool>,
    path: web::Path<(u32,)>,
) -> Result<HttpResponse, ResErr> {
    let mut user = (execute(&db, Queries::GetUserById(path.into_inner().0))
        .map_err(|_| ResErr::BadClientData("cant get user"))?)
    .one();

    user.pass = "".to_string();

    Ok(HttpResponse::Ok().json(user))
}

pub async fn add_user(
    _: MustAdminOrOp,
    db: web::Data<Pool>,
    mut user: web::Json<User>,
) -> Result<HttpResponse, ResErr> {
    if user.status == 1 {
        return Err(ResErr::BadClientData("cant create admin"));
    }

    // error message dont work
    user.validate()
        .map_err(|err| ResErr::BadClientData("no valid input"))?;

    user.pass =
        hash(user.pass.clone(), DEFAULT_COST).map_err(|_| ResErr::InternalError("bad hash"))?;

    // error message dont work
    (execute(&db, Queries::AddUser(user)).map_err(|_| ResErr::BadClientData("cant add user"))?)
        .none();

    Ok(HttpResponse::Ok().body("user added"))
}

pub async fn delete_user(
    _: MustAdminOrOp,
    db: web::Data<Pool>,
    path: web::Path<(u32,)>,
) -> Result<HttpResponse, ResErr> {
    let id = path.into_inner().0;

    let user_stat = (execute(&db, Queries::GetUserById(id))
        .map_err(|_| ResErr::BadClientData("cant get user"))?)
    .one();

    if user_stat.status == 1 {
        return Err(ResErr::BadClientData("cant delete admin"));
    };

    (execute(&db, Queries::DeleteUserById(id)).map_err(|err| {
        println!("{:?}", err);
        ResErr::BadClientData("cant delete user")
    })?)
    .none();

    Ok(HttpResponse::Ok().body("user deleted"))
}

pub async fn update_user(
    _: MustAdminOrOp,
    db: web::Data<Pool>,
    path: web::Path<(u32,)>,
    mut user: web::Json<User>,
) -> Result<HttpResponse, ResErr> {
    let id = path.into_inner().0;

    // error message dont work
    user.validate()
        .map_err(|err| ResErr::BadClientData("no valid input"))?;
    if user.status == 1 {
        return Err(ResErr::BadClientData("cant add admin"));
    };

    let user_stat = (execute(&db, Queries::GetUserById(id))
        .map_err(|_| ResErr::BadClientData("cant get user"))?)
    .one();

    if user_stat.status == 1 {
        return Err(ResErr::BadClientData("cant update admin"));
    };

    user.pass =
        hash(user.pass.clone(), DEFAULT_COST).map_err(|_| ResErr::InternalError("bad hash"))?;

    // error message dont work
    (execute(&db, Queries::UpdateUserById(id, user))
        .map_err(|_| ResErr::BadClientData("cant update user"))?)
    .none();

    Ok(HttpResponse::Ok().body("user updated"))
}
