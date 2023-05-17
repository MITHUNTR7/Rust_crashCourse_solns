// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
struct person{
    age : i32,
    name : String,
    color : String
}

fn print_it(name : &str, color : &str){
    println!("Name : {:?}",name);
    println!("Color : {:?}",color);
}



fn main() {
    
    let People = vec![
        person{
            age : 10,
            name : "Steve".to_owned(),
            color : String::from("Blue"),
        },
        person{
            age : 17,
            name : "Kevin".to_owned(),
            color : String::from("Red"),
        },
        person{
            age : 9,
            name : "Erwin".to_owned(),
            color : String::from("green"),
        },
    ];
    
    for people in People{
        if people.age <= 10{
            print_it(&people.name, &people.color);
        }
    }
    
    
}
