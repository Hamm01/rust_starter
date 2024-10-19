/* For generating the uuid
Adding something in dependices in cargo.toml file to run the new_v4 function
[dependencies]
chrono = "0.4.32"
dotenv = "0.13.0"
uuid = { version = "1.4", features = ["v4"] }

*/
use uuid::Uuid;
fn main() {
    let id = Uuid::new_v4();
    println!("{}", id);
}
