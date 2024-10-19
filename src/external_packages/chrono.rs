/*
We are using the Chrono library
we brought the extrernal crate to our project which is chrono , we added into cargo.toml file
[dependencies]
chrono = "0.4.35"

*/

use chrono::{Local, Utc};

fn main() {
    let utc = Utc::now();
    let local = Local::now(); // This will give us the local time
    print!("{}", utc);
    print!("{}", local);
}
