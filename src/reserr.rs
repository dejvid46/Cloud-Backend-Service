use actix_web::{
    dev::HttpResponseBuilder, http::header, http::StatusCode, error::ResponseError, HttpResponse,
};
use std::{fmt::{Display, Formatter}};

#[derive(Debug)]
pub enum ResErr {
    InternalError(&'static str),
    BadClientData(&'static str),
}

impl Display for ResErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            &ResErr::InternalError(s) => write!(f, "{}", s),
            &ResErr::BadClientData(s) => write!(f, "{}", s),
        }
    }
}

impl ResponseError for ResErr {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ResErr::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ResErr::BadClientData(_) => StatusCode::BAD_REQUEST,
        }
    }
}