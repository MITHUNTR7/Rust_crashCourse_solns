// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct student{
    name : String,
    locker_num : Option<i32>
}



fn main() {
    
    let Student = student{
        name : "Caleb".to_owned(),
        locker_num : Some(7),
    };
    ///Advanced match statement used to match the option data type
    match Student.locker_num{
        Some(num) => println!("The locker assignment is {:?}",num),
        None => print!("No locker assignment")
    }
    
    
    
    
    
    
}
