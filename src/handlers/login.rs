use actix_web::{HttpResponse, web, HttpRequest};
use bcrypt::verify;

use crate::models::{Queries, Login};
use crate::db::{execute, Pool};
use crate::jwt::{create_jwt, authorize};
use crate::reserr::ResErr;

pub async fn login(db: web::Data<Pool>, user: web::Json<Login>) -> Result<HttpResponse, ResErr> {

    let res = (execute(&db, Queries::GetUserByEmail(user.email.clone()))
        .map_err(|_| ResErr::BadClientData("bad email or password"))?).one();

    let verify_res = verify(&user.pass, &res.pass).map_err(|_| ResErr::BadClientData("bad email or password"))?;

    if !verify_res {
        return Err(ResErr::BadClientData("bad email or password"));
    }
    
    Ok(HttpResponse::Ok().json(create_jwt(res.id)))
}

pub async fn check_login(req: HttpRequest) -> Result<HttpResponse, ResErr> {
    let claims = authorize(&req).map_err(|err| ResErr::BadClientData(err))?;

    Ok(HttpResponse::Ok().json(claims))
}