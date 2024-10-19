/*

Dotenv packages used in project to check the enviroment variables that
need to be used in projects

cargo add dotenv
*/

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let var = env::var("JWT_SECRET").unwrap();

    match var {
        Ok(str) => println!("{}", str),
        Err(_e) => print!("Error while reading variable"),
    }
}
