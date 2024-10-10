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



/*
struct Rect {
  width: f32,
  height: f32,
}
impl Rect{
// how we define the member functions
   fn area(&self) -> f32{
      return self.width * self.height
  }
  fn printsome(){
  // this is similar to static function in js
    println!("print Something this is static function")
  }
}  

fn main(){
   let r = Rect{
      width: 10.0,
      height:10.0,
    };
    println!("{} {}", r.width, r.height);
    println!("{} ", r.area()); // calling the member function
    Rect::printsome(); // calling the static function
}
*/
