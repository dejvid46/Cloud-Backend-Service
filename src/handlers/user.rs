use actix_web::{web, HttpResponse};
use bcrypt::{hash, DEFAULT_COST};
use validator::Validate;

use crate::db::{execute, Pool};
use crate::middleware::MustLogin;
use crate::models::{ChangingUser, Queries};
use crate::reserr::ResErr;

pub async fn get_me(token: MustLogin, db: web::Data<Pool>) -> Result<HttpResponse, ResErr> {
    let mut user = (execute(&db, Queries::GetUserById(token.id))
        .map_err(|_| ResErr::BadClientData("cant get user"))?)
    .one();

    user.pass = "".to_string();

    Ok(HttpResponse::Ok().json(user))
}

pub async fn update_me(
    token: MustLogin,
    db: web::Data<Pool>,
    mut user: web::Json<ChangingUser>,
) -> Result<HttpResponse, ResErr> {
    // error message dont work
    user.validate().map_err(|err| {
        ResErr::BadClientDataOwned(
            err.field_errors().into_values().next().unwrap()[0]
                .code
                .as_ref()
                .to_string(),
        )
    })?;

    user.pass =
        hash(user.pass.clone(), DEFAULT_COST).map_err(|_| ResErr::InternalError("cant hash"))?;

    (execute(&db, Queries::UpdateMeById(token.id, user))
        .map_err(|_| ResErr::BadClientData("cant update"))?)
    .none();

    Ok(HttpResponse::Ok().body("updated"))
}
