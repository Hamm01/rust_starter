// Enums are seemingly similar to enum in rust
// how to define an enum

enum Color{
    Red,
    Yellow,
    White,
    Black,
    Green,
  }
  
  fn main() {
    let newColor = Color::White;
    refinecolor(newColor);

  }
  
  fn refinecolor(new:Color){

    // This is not working code but to take an idea how its works, 
     if(new == Color::White){
       println!("Color to refine is white");
      }
      if(new == Color::Black){
        println!("Color to refine is Black");
       }
      if(new == Color::Red){
        println!("Color to refine is Red");
       }
       if(new == Color::Yellow){
        println!("Color to refine is Yellow");
       }
       if(new == Color::Green){
        println!("Color to refine is Green");
       }
  }