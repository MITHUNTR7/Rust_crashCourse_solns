// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum colors{
    red,
    green,
    blue,
    yellow,
    purple
}

fn print_color (color:colors)
{
    match color{
        colors::red => println!("The color is red"),
        colors::green => println!("The color is green"),
        colors::blue => println!("The color is blue"),
        colors::yellow => println!("The color is yellow"),
        colors::purple => println!("The color is purple"),
        
    }
}

fn main() {
    
    let my_color = colors::purple;
    
    print_color(my_color);
    
    
}
