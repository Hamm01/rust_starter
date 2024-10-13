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


/*

 // Enums with the values we can pass and checking the 
enum Shape{
  Circle(f32),
  Rectangle(f32,f32),
  Square(f32),
  }

fn main(){

    let rectangle_shape =  Shape::Rectangle(12.0,12.0);
    print!("{}",findarea(rectangle_shape));

}

fn findarea(shape: Shape) -> f32 {
   
   match shape{
    Shape::Rectangle(width,height)  =>  width*height,
    Shape::Circle(radius)  =>  std::f32::consts::PI * radius * radius,
    Shape::Square(side ) => side * side,
  }
  

}
*/


/*
// Implementing Enums 
enum Shape{
  Circle(f32),
  Rectangle(f32,f32),
  Square(f32),
  }


  impl Shape{
       fn area(&self) -> f32{
        match self{
          Shape::Rectangle(width,height)  =>  width*height,
          Shape::Circle(radius)  =>  std::f32::consts::PI * radius * radius,
          Shape::Square(side ) => side * side,
        }
       }
  }

fn main(){

    let rectangle_shape =  Shape::Rectangle(12.0,12.0);
    print!("{}",rectangle_shape.area());  // this can directly called area function

}



*/