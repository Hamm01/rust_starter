/*

  Optiomal enum in rust, this is introduced in rust to solve the concept of null values,, 
  in rust we dont have the null values. 

  pub enum Option<T>{
  None,
  Some(T),
  }

*/

// For a function that will return null , we return option enum instead

fn main(){
  let result =  find_first_a(String::from("Himanish"));
  // we will find the first occurence of a in string, and in case string not have a then the function will pass none instead of null
  match result{
      None => print!("Value not found!"),
      Some(val) => print!("a  is at index {}",val)
  }
}


fn find_first_a(str: String)  -> Option<u32>{

  let mut index:u32 = 0;
  for c in str.chars(){
      if c == 'a'{
         return Some(index)
      }
      index = index +1;
  }
 return None;

}