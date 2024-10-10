// structs like object in javascript that will need to structure the data together


struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    // how to declare the struct
}


fn main(){
    let userone = User{
      active: true,
      username: String::from("Himanish"),
      email: String::from("example@gma.com"),
      sign_in_count: 1,
    };
    print!("User One username is {}", userone.username);

}
