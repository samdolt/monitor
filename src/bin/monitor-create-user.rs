extern crate monitor;
extern crate diesel;

use self::monitor::db::models::User;
use std::io::stdin;

use self::monitor::db::*;

fn main() {
    let connection = establish_connection();

    println!("Name?");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)]; // Drop the newline character

    println!("Email?");
    let mut email = String::new();
    stdin().read_line(&mut email).unwrap();
    let email = &email[..(email.len() -1)];

    println!("Password?");
    let mut password = String::new();
    stdin().read_line(&mut password).unwrap();
    let password = &password[..(password.len()-1)];

    let user = User::new(&connection, name , email, password).unwrap();
    println!("\nSaved user {} with id {}", name, user.id);
}

