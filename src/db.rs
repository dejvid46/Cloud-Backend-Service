use actix_web::web::Json;
use bcrypt::{hash, DEFAULT_COST};
use rusqlite::NO_PARAMS;

use crate::models::{ChangingUser, Queries, User};

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;
use r2d2_sqlite::{self};

pub enum QueryResult<T> {
    Many(Vec<T>),
    One(T),
    None(()),
}

impl<T> QueryResult<T> {
    pub fn many(self) -> Vec<T> {
        match self {
            QueryResult::Many(t) => t,
            _ => unreachable!(),
        }
    }
    pub fn one(self) -> T {
        match self {
            QueryResult::One(t) => t,
            _ => unreachable!(),
        }
    }
    pub fn none(self) -> () {
        match self {
            QueryResult::None(t) => t,
            _ => unreachable!(),
        }
    }
}

pub fn execute(pool: &Pool, query: Queries) -> Result<QueryResult<User>, rusqlite::Error> {
    let pool = pool.clone();
    Ok(match query {
        Queries::GetUserById(id) => QueryResult::One(get_user_by_id(
            pool.get()
                .map_err(|_| rusqlite::Error::QueryReturnedNoRows)?,
            id,
        )?),
        Queries::GetUserByEmail(email) => QueryResult::One(get_user_by_email(
            pool.get()
                .map_err(|_| rusqlite::Error::QueryReturnedNoRows)?,
            email,
        )?),
        Queries::GetAllUsers => QueryResult::Many(get_all_users(
            pool.get()
                .map_err(|_| rusqlite::Error::QueryReturnedNoRows)?,
        )?),
        Queries::AddUser(user) => QueryResult::None(add_user(
            pool.get()
                .map_err(|_| rusqlite::Error::QueryReturnedNoRows)?,
            &user,
        )?),
        Queries::DeleteUserById(id) => QueryResult::None(delete_user_by_id(
            pool.get()
                .map_err(|_| rusqlite::Error::QueryReturnedNoRows)?,
            id,
        )?),
        Queries::UpdateUserById(id, user) => QueryResult::None(update_user_by_id(
            pool.get()
                .map_err(|_| rusqlite::Error::QueryReturnedNoRows)?,
            id,
            &user,
        )?),
        Queries::UpdateMeById(id, user) => QueryResult::None(update_me_by_id(
            pool.get()
                .map_err(|_| rusqlite::Error::QueryReturnedNoRows)?,
            id,
            &user,
        )?),
    })
}

fn get_user_by_id(conn: Connection, id: u32) -> Result<User, rusqlite::Error> {
    conn.prepare(
        "
        SELECT *
        FROM Users
        WHERE id=(?1)
        LIMIT 1
    ",
    )?
    .query_row(&[&id], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            pass: row.get(3)?,
            size: row.get(4)?,
            path: row.get(5)?,
            status: row.get(6)?,
        })
    })
}

fn get_user_by_email(conn: Connection, email: String) -> Result<User, rusqlite::Error> {
    conn.prepare(
        "
        SELECT *
        FROM Users
        WHERE email=(?1)
        LIMIT 1
    ",
    )?
    .query_row(&[&email], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            pass: row.get(3)?,
            size: row.get(4)?,
            path: row.get(5)?,
            status: row.get(6)?,
        })
    })
}

fn get_all_users(conn: Connection) -> Result<Vec<User>, rusqlite::Error> {
    conn.prepare(
        "
        SELECT *
        FROM Users
    ",
    )?
    .query_map(NO_PARAMS, |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            pass: row.get(3)?,
            size: row.get(4)?,
            path: row.get(5)?,
            status: row.get(6)?,
        })
    })
    .and_then(Iterator::collect)
}

fn add_user(conn: Connection, user: &Json<User>) -> Result<(), rusqlite::Error> {
    conn.prepare(
        "
        INSERT INTO Users (name, email, pass, size, path, status)
        VALUES(?1, ?2, ?3, ?4, ?5, ?6)
    ",
    )?
    .execute(&[
        &user.name,
        &user.email,
        &user.pass,
        &user.size.to_string(),
        &user.path,
        &user.status.to_string(),
    ])?;
    Ok(())
}

fn delete_user_by_id(conn: Connection, id: u32) -> Result<(), rusqlite::Error> {
    conn.prepare(
        "
        DELETE 
        FROM Users
        WHERE id=(?1)
    ",
    )?
    .execute(&[&id])?;
    Ok(())
}

fn update_user_by_id(conn: Connection, id: u32, user: &Json<User>) -> Result<(), rusqlite::Error> {
    conn.prepare(
        "
        UPDATE Users 
        SET 
            name = ?2, 
            email = ?3, 
            pass = ?4, 
            size = ?5, 
            path = ?6, 
            status = ?7
        WHERE id=(?1)
    ",
    )?
    .execute(&[
        &id.to_string(),
        &user.name,
        &user.email,
        &user.pass,
        &user.size.to_string(),
        &user.path,
        &user.status.to_string(),
    ])?;
    Ok(())
}

fn update_me_by_id(
    conn: Connection,
    id: u32,
    user: &Json<ChangingUser>,
) -> Result<(), rusqlite::Error> {
    conn.prepare(
        "
        UPDATE Users 
        SET 
            name = ?2, 
            email = ?3, 
            pass = ?4
        WHERE id=(?1)
    ",
    )?
    .execute(&[&id.to_string(), &user.name, &user.email, &user.pass])?;
    Ok(())
}

pub fn create_tables(conn: &Pool) {
    conn.get()
        .unwrap()
        .execute(
            "create table if not exists Users (
            id integer primary key,
            name VARCHAR(10) NOT NULL UNIQUE,
	        email VARCHAR(30) NOT NULL UNIQUE,
            pass TEXT NOT NULL,
            size UNSIGNED INT,
	        path TEXT NOT NULL,
            status UNSIGNED TINYINT)
        ",
            NO_PARAMS,
        )
        .unwrap();
    conn.get()
        .unwrap()
        .execute(
            "
            INSERT INTO Users (name, email, pass, size, path, status) 
            SELECT ?1, ?2, ?3, ?4, ?5, ?6
            WHERE NOT EXISTS(SELECT 1 FROM Users WHERE status = 1)
        ",
            &[
                "admin",
                "admin@admin.com",
                &hash("admin", DEFAULT_COST).unwrap(),
                &*u32::MAX.to_string(),
                "/",
                "1",
            ],
        )
        .unwrap();
}
