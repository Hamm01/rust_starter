// How to read file content  in rust

use std::fs;


fn main(){

   let contents =  fs::read_to_string("a.txt");
   match contents{
     Ok(contents) => println!("{}",contents),
     Err(_e) => print!("Error reading the file")
    
  }
}