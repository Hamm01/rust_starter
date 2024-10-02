fn main() {
    let x: i32 = -23;   // this will used for postive or negative, last bit is reserved for saving post or negtive value
    println!("{}", x);
}

// u32 hold unsigned integer value, the last bit will also used to store the whole number , not for postive or negative 
// 8, 32, 64, 128, these are all sizes in that can store numbers as acoording to space,

fn main() {
    let is_male = false;
    let is_above_18 = true;
    // how booleans arre stored
    if is_male {
        println!("You are a male");

    } else {
        println!("You are not a male");
    }

    if is_male && is_above_18 {
        print!("You are a legal male");
    }
}