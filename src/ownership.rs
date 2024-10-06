// Ownership is a set of rules that govern how a Rust program manages memory.
/*
Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks.
 If any of the rules are violated, the program won’t compile. 
 None of the features of ownership will slow down your program while it’s running.
*/

fn main() {
    let str = String::from("Himanish");
    let len = get_length(str); // str is passed to get length function
    println!("{}", len);
  
    print!("{}", str); // here this str we cannot access,. it will throw error in compilation
  }
  
  fn get_length(str: String) -> usize {
    return str.len()
  } 