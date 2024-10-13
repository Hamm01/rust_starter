/*
 Like in js we use try catch block to catch the errors  in rust we have enum called Result
 that returnd by when we use like fs , or reading the file contents

 Result Enum is looks like
 enum Result<T, E> {
    Ok(T),
    Err(E),
}
This enum is what a function can return/returns when it has a possibility of throwing an error
*/

use std::fs;
enum Result{
    Ok(String),
    Err(Error),
}



fn main() {
    let greeting_file_result = fs::read_to_string("a.txt");

    match greeting_file_result {
        Ok(file_content) => {
            println!("File read successfully: {:?}", file_content);
        },
        Err(error) => {
            println!("Failed to read file: {:?}", error);
        }
    }
}