#![allow(non_snake_case)]
use std::process::exit;
use text_io::read;
use crate::models;
use models::*;
use crate::handlers::{InputItem, ReadCommand};

pub fn OpenUserPanel(user:&mut User, restaurants:&mut Vec<Restaurant>){
    let displayString="here is user panel: \n\
    please select:\n\
    make new order: 1\n\
    display orders: 2".to_string();
    println!("{displayString}");
    let mut command=ReadCommand();
    while command!=9 {
        match command {
            1=>{
                let order=Order::default();
                let mut selectedRestaurant=OpenRestaurantSelectionPanel(restaurants);
                if(OpenRestuarntItemsSelectionPanel(&mut selectedRestaurant.unwrap(),user)){
                    user.orders.push(order);
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
        println!("{displayString}");
        command=ReadCommand();
    }
}


pub fn OpenRestaurantAdminPanel(restuarant:&mut Restaurant){
    let displayString="here is restaurant panel\n\
    please select:\n\
    add item into menu: 1\n\
    exit: 9".to_string();
    println!("{displayString}");
    let mut command=ReadCommand();
    while command!=9 {
        match command {
            1=>{
                println!("please enter item name and price:");
                let item=InputItem();
                restuarant.items.push(item);
            }
            _=>{
                println!("invalid command!");
            }
        }
        println!("{displayString}");
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
        println!("{} {:?}\n",n.index,n);
    }
    println!("enter {} to exit",restaurants.len());
    let command=ReadCommand();
    if(command==restaurants.len() as i32){
        println!("aborting...");
        return None
    }
    Some(&mut restaurants[command as usize])
}
pub fn OpenRestuarntItemsSelectionPanel(restaurant: &mut Restaurant,user:&mut User)->bool{
    if(restaurant.items.len()==0){
        println!("restaurant has no item available!");
        return false;
    }
    println!("here is items available for this restaurant. please select one:");
    for (i,n) in restaurant.items.iter().enumerate() {
        println!("{} {:?}",i,n);
    }
    println!("enter {} to exit",restaurant.items.len());
    let command=ReadCommand();
    if(command as usize==restaurant.items.len()){
        println!("aborting...");
        return false;
    }
    let selectedItem:&Item=&restaurant.items[command as usize];
    let order=Order{
        userIndex:user.index,
        restaurantIndex:restaurant.index,
        item:selectedItem.clone(),
        orderStatus:OrderStatus::inCart,
    };
    restaurant.orders.push(order);
    true
}