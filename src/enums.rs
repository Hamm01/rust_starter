// Enums are seemingly similar to enum in rust
// how to define an enum

// Pattern matching using enums

enum Color{
    Red,
    Yellow,
    White,
    Black,
    Green,
  }
  
  fn main() {
    let newColor = Color::Red;
    refinecolor(newColor);
  
  }
  
  fn refinecolor(new : Color){
  match new{
    // Pattern matching using enums

       Color::Black => print!("Black color to refine"),
       Color::White => print!("White color to refine"),
       _ => print!("Colors other than white and black")
  }
  
  }