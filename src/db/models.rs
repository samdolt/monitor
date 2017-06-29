use ::diesel::pg::PgConnection;
use ::diesel;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::Connection;

use djangohashers::check_password;
use djangohashers::make_password;

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
            password: &make_password(password),
        };

        diesel::insert(&new_user).into(users::table)
            .get_result(conn)
    }

    pub fn get_user(conn: &PgConnection, email: &str) -> Result<User, Error> {
        use super::schema::users::dsl::*;

        Ok(users.filter(email.eq(email)).first::<User>(conn)?)
    }

    pub fn check_password(&self, password: &str) -> bool {
        if let Ok(pw) = check_password(password, &self.password) {
           true
        } else {
            false
        }
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