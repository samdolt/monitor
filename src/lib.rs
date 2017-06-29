extern crate influxdb;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

extern crate dotenv;
extern crate subcmd;
extern crate djangohashers;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod db;
pub mod waspmote;
pub mod commands;




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
