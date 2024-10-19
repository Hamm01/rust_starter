/*

Dotenv packages used in project to check the enviroment variables that
need to be used in projects

cargo add dotenv
*/

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let var = env::var("JWT_SECRET").unwrap(); // here var will be of String if the env variable exist otherwise it will throw error

    match var {
        Ok(str) => println!("{}", str),
        Err(_e) => print!("Error while reading variable"),
    }
}

fn main() {
    dotenv().ok();
    let var = env::var("JWT_SECRET"); // here var be of result enum type : Result<String, env::VarError>

    match var {
        Ok(str) => println!("{}", str),
        Err(_e) => print!("Error while reading variable"),
    }
}
