fn main(){
  
  let arr: [i32; 5] = [2,3,4,5,6];
  println("{}",arr.len());
  println("{}",arr);
}

//Vectors declaration

fn main() {
    let mut xs = vec![1, 2, 3];
    
    print!("{}", xs.len());

    xs.push(4);
    
    print!("{}", xs.len());
}