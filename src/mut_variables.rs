// This is concept for Mutable vs immutable variables
// By default, all variables in rust are imutable. 


fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x isa {x}");

    // This code will not compile as we are mutating the value of x
}