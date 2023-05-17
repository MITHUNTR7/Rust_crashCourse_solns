// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info
enum tickets{
    Backstage(i32,String),
    Vip(i32,String),
    Standard(i32)
}




fn main() {
    
    let Tickets = vec![
        tickets::Backstage(50,"Austin".to_owned()),
        tickets::Vip(75,"Theo".to_owned()),
        tickets::Standard(25),
    ];
    for tick in Tickets{
        match tick{
            tickets::Backstage(amount, name) => println!("Name : {:?}, Backstage ticket, price : {:?}",name,amount),
            tickets::Vip(amount, name) => println!("Name : {:?}, Vip ticket, price : {:?}",name,amount),
            tickets::Standard(amount,) => println!("Standard ticket, price : {:?}",amount),
            
        }
    }
    
    
    
    
    
    
    
}
