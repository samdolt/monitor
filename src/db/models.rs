use ::diesel::pg::PgConnection;
use ::diesel;
use diesel::prelude::*;
use diesel::result::Error;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    password: String,
    pub enabled: bool,
}

impl User {

    pub fn new(conn: &PgConnection,name: &str, email: &str, password: &str) -> Result<User, Error> {
        use super::schema::users;

        let new_user = NewUser {
            name: name,
            email: email,
            password: password,
        };

        diesel::insert(&new_user).into(users::table)
            .get_result(conn)
    }
}

use super::schema::users;

#[derive(Insertable)]
#[table_name="users"]
struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}