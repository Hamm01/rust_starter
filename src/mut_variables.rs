// This is concept for Mutable vs immutable variables
// By default, all variables in rust are imutable. 


fn main() {
    let x = 9;
    println!("The value of x is: {x}");
    x = 10;
    println!("The value of x is {x}");

    // This code will not compile as we are mutating the value of x
}


fn main() {
    let mut x = 10;
    println!("The value of x is: {x}");
    x = 20;
    println!("The value of x is  {x}");

    // this code will compile as according the rust. because we need to mannualy declare mut before variable.
}