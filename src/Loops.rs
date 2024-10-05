fn main(){
    // Simple loop to print numbers
    for i in 0..100{
    println!("{}",i);
  }
}

fn main(){
  // Loop for string
    let str = String::from("Himanish kochhar");
    println!("firstname {}", get_first_name(str));

}

pub fn get_first_name(str: String) -> String{
       let mut firstname = String::from("");

       for c in str.chars(){
           if c == ' '{
          break }
          firstname.push(c);
       }
       return firstname;

}