// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock
use std::collections::HashMap;
fn main() {
    
    
    let mut items = HashMap::new();
    
    items.insert(5, "Chairs");
    items.insert(3, "Beds");
    items.insert(2, "Tables");
    items.insert(0, "Couches");
    
    let mut item_number = 0;
    
    for (stock,item) in items.iter(){
        
        item_number = item_number + stock;
        
        if *stock == 0{
            println!("Couches are out of stock");
            
        }
        else{
            println!("{:?} stock = {:?}",item,stock);
            
        }
    }
    
    println!("Number of items : {:?}",item_number);
    
    
    
    
    
    
    
}
