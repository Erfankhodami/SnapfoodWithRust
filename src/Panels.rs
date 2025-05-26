#![allow(non_snake_case)]

use std::env::args_os;
use std::net::Shutdown::Read;
use std::process::exit;
use std::ptr::addr_eq;
use text_io::read;
use crate::models;
use models::*;
use crate::handlers::{InputItem, ReadCommand, SaveUsersToJson};

pub fn OpenUserPanel(user:&mut User, restaurants:&mut Vec<Restaurant>){
    let displayString="here is user panel: \n\
    please select:\n\
    make new order: 1\n\
    display orders: 2\n\
    remove order: 3\n\
    pay order: 4\n\
    charge wallet: 5\n\
    exit: 9".to_string();
    println!("{displayString}");
    let mut command=ReadCommand();
    while command!=9 {
        match command {
            1=>{
                let mut selectedRestaurant=OpenRestaurantSelectionPanel(restaurants);
                let result=OpenRestaurantItemsSelectionPanel(&mut selectedRestaurant.unwrap(),user);
                if(result.is_some()){
                    user.orders.push(result.unwrap());
                    println!("order added successfully!");
                }
            }
            2=>{
                println!("here are your orders:");
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
            3=>{
                if(user.orders.len()==0){
                    println!("no orders found!");
                }else {
                    println!("here are your orders:\nselect one to remove\nselect {} to exit", user.orders.len());
                }
                for (i,n) in user.orders.iter().enumerate() {
                    println!("{} {:?}",i,n);
                }
                let command=ReadCommand();
                if command!=user.orders.len() as i32{
                    user.orders.remove(command as usize);
                    println!("order removed successfully!");
                }else{
                    println!("aborting...");
                }
            }
            4=>{
                if let Err(e)=OpenOrderPayPanel(user){
                    println!("{e}");
                }
            }
            5=>{
                println!("input amount you wanna charge your wallet:");
                let amount=ReadCommand();
                user.wallet+=amount as u64;
            }
            _=>{
                println!("invalid command");
            }
        }
        println!("{displayString}");
        command=ReadCommand();
    }
}

pub fn OpenOrderPayPanel(user:&mut User)->Result<(),String>{
    println!("select an order to pay or select {} to exit",user.orders.len());
    for (i,n) in user.orders.iter().enumerate() {
        match n.orderStatus {
            OrderStatus::inCart=>{
                println!("{} {:?}",i,n);
            }
            _=>{}
        }

    }
    let command=ReadCommand() as usize;
    if(command==user.orders.len()){
        return Err("order didnt paid!".to_string());
    }
    let orderPrice=user.orders[command].item.price;
    if(orderPrice>user.wallet){
        return Err("not enough cash to pay this order!".to_string());
    }
    user.wallet-=orderPrice;
    user.orders[command].orderStatus=OrderStatus::onWay;
    println!("order paid successfully. order is on way!");
    Ok(())
}
pub fn OpenRestaurantAdminPanel(restuarant:&mut Restaurant){
    let displayString="here is restaurant panel\n\
    please select:\n\
    add item into menu: 1\n\
    view items: 2\n\
    view orders: 3\n\
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
            2=>{
                println!("here are items available in your restaurant:");
                for (i,n) in restuarant.items.iter().enumerate() {
                    println!("{} {:?}",i,n);
                }
            }
            3=>{
                println!("here are your orders:");
                for (i,n) in restuarant.orders.iter().enumerate() {
                    println!("{} {:?}",i,n);
                }
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
pub fn OpenRestaurantItemsSelectionPanel(restaurant: &mut Restaurant,user:&mut User)->Option<Order>{
    if(restaurant.items.len()==0){
        println!("restaurant has no item available!");
        return None;
    }
    println!("here is items available for this restaurant. please select one:");
    for (i,n) in restaurant.items.iter().enumerate() {
        println!("{} {:?}",i,n);
    }
    println!("enter {} to exit",restaurant.items.len());
    let command=ReadCommand();
    if(command as usize==restaurant.items.len()){
        println!("aborting...");
        return None;
    }
    let selectedItem:&Item=&restaurant.items[command as usize];
    let order=Order{
        userIndex:user.index,
        restaurantIndex:restaurant.index,
        item:selectedItem.clone(),
        orderStatus:OrderStatus::inCart,
    };
    restaurant.orders.push(order.clone());
    Some(order)
}