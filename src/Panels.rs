use std::process::exit;
use text_io::read;
use crate::models;
use models::*;
use crate::handlers::ReadCommand;

pub fn OpenUserPanel(user:&mut User, restaurants:&mut Vec<Restaurant>){
    println!("here is user panel: \nplease select:\nmake new order: 1\ndisplay orders: 2");
    let mut command=ReadCommand();
    while command!=9 {
        match command {
            1=>{
                let order=Order::default();
                let mut selectedRestaurant=OpenRestaurantSelectionPanel(restaurants);
                if(OpenRestuarntItemsSelectionPanel(&mut selectedRestaurant.unwrap(),user)){
                    println!("order added successfully!");
                }
            }
            2=>{
                let result=user.DisplayOrders();
                match result {
                    Some(data)=>{
                        println!("{data}");
                    }
                    None=>{
                        println!("no orders found!");
                    }
                }
            }
            _=>{
                println!("invalid command");
            }
        }
        println!("here is user panel: \nplease select:\nmake new order: 1");
        command=ReadCommand();
    }
}
pub fn OpenRestaurantSelectionPanel(restaurants:&mut Vec<Restaurant>)->Option<&mut Restaurant>{
    if(restaurants.len()==0){
        println!("no restaurants available!");
        return None
    }
    println!("here is the restaurants available. please select one:");
    for n in restaurants.iter() {
        println!("{:?}",restaurants);
    }
    let command=ReadCommand();
    Some(&mut restaurants[command as usize])
}
pub fn OpenRestuarntItemsSelectionPanel(restaurant: &mut Restaurant,user:&mut User)->bool{
    if(restaurant.items.len()==0){
        println!("restaurant has no item available!");
        return false;
    }
    println!("here is items available for this restaurant. please select one:");
    for n in &restaurant.items {
        println!("{:?}",n);
    }
    println!("{}: exit",restaurant.items.len());
    let command=ReadCommand();
    if(command as usize==restaurant.items.len()){
        println!("aborting...");
    }
    let selectedItem:&Item=&restaurant.items[command as usize];
    let order=Order{
        userIndex:user.index,
        restaurantIndex:restaurant.index,
        item:selectedItem.clone(),
        orderStatus:OrderStatus::inCart,
    };
    restaurant.orders.push(order);
    return true;
}