// We are using the Chrono library 
use chrono::Utc;

/*
we brought the extrernal crate to our project which is chrono , we added into cargo.toml file
[dependencies]
chrono = "0.4.35"

*/


fn main(){
   let utc = Utc::now();
   print!("{}",utc);
}