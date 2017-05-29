extern crate monitor;
extern crate diesel;

use monitor::db::models::*;
use self::diesel::prelude::*;

fn main() {
    use monitor::db::schema::users::dsl::*;

    let db = monitor::db::establish_connection();

    let results = users.filter(enabled.eq(true))
                       .load::<User>(&db)
                       .expect("Can't load users");

    for user in results {
        println!("Found user {} with email {}", user.name, user.email);
    }
}