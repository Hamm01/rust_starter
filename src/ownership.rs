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


  // code fixed 

  fn main() {
    let str = String::from("Vikas sharma");
    let (str, len) = get_length(str);
    println!("{} {}", str, len);
}

fn get_length(str: String) -> (String, usize) {
    let len = str.len();
    return (str, len);
}


/*
fn main() {
  let str = String::from("Himanish");
  let ref1 = &str;
  let ref2 = &str;
                      // this code will work properly
  println!("{} {}", ref1, ref2);
}

  fn main() {
    let mut str = String::from("Himanish");
    let ref1 = &mut str;
    let ref2 = &str;
          // the compilation error comes with immutable borrow occurs here , if one mutable refrence made then we cannot make another
          // imutable refrence 
    println!("{} {}", ref1, ref2);
}

fn main() {
    let mut str = String::from("Himanish");
    let ref1 = &mut str;
    ref1.push_str("koc");
    let ref2 = &str;
     // this code will work propery because we are using ref1 afterwords in code
    println!("{}", ref2);
}
*/