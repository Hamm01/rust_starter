// When you pass a variable by reference, 
// the variable is still owned by the first function. It is only borrowed by the get_length function.

fn main() {
    let s1 = String::from("Himanish");
    let len = get_length(&s1); // str2 is borrowing the s1
    println!("{}",len);
    get_length(&s1); // now we can use s1 as many times, because it ownership is not passing
    get_length(&s1);
  }
  
  fn get_length(str2: &String) -> usize {
   return str2.len();
  }