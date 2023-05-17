// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
enum Color{
    red,
    green,
    blue
}

struct Dimensions{
    length: i32,
    width: i32,
    height: i32,
}

struct Box{
    weight : f64,
    color : Color,
    dimensions : Dimensions
}

impl Box{
    fn create_box(box_weight: f64, box_color: Color, box_dimensions: Dimensions) -> Self{
        Self { weight: box_weight, color: box_color, dimensions: box_dimensions }
    }
    
    fn print_box(&self){
        println!("weight : {:?}",self.weight);
        println!("length : {:?}",self.dimensions.length);
        println!("width : {:?}",self.dimensions.width);
        println!("height : {:?}",self.dimensions.height);
        
        match self.color{
            
            Color::blue => println!("Color : blue"),
            Color::green => println!("Color : green"),
            Color::red => println!("Color : red"),
        }
    }
    
}

fn main() {
    
   let my_box = Box::create_box(70.0, Color::blue, Dimensions{length:5,width:6,height:7});
   my_box.print_box();
    
}
