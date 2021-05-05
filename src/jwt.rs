use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use actix_web::HttpRequest;
use std::env;
use chrono::Utc;
use crate::models::{Claims, Token};

pub fn create_jwt(id: u32) -> Token {

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(env::var("TOKEN_EXPIRATION").unwrap().parse().unwrap()))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        id: id,
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    
    Token {token: encode(&header, &claims, &EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes())).unwrap()}
}

pub fn authorize(req: &HttpRequest) -> Result<Claims, &'static str> {

    if let Some(jwt) = get_content_type(&req) { 
        match decode::<Claims>(
            &jwt,
            &DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes()),
            &Validation::new(Algorithm::HS512),){
            Ok(decoded) => Ok(decoded.claims),
            Err(_) => Err("invalid token")
        }
    } else {
        Err("token not found")
    }
}

fn get_content_type<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("token")?.to_str().ok()
}