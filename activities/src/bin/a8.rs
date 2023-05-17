// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor



enum Flavours{
    apple,
    grape,
    orange
}

struct Drink{
    flavour: Flavours,
    ounces : f64,
}

fn print_drink(drink:Drink)
{
    match drink.flavour{
        Flavours::apple => println!("Flavour : apple"),
        Flavours::grape => println!("Flavour : grape"),
        Flavours::orange => println!("Flavour : orange"),
    }
    
    println!("Ounces : {:?}",drink.ounces);
}




fn main() {
    
    let my_drink = Drink{
        flavour: Flavours::apple,
        ounces: 8.09
    };
    
    print_drink(my_drink)
    
    
   
    
}
