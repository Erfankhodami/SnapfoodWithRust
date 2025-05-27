#![allow(non_snake_case)]

use std::env::args_os;
use std::net::Shutdown::Read;
use std::ops::Index;
use std::process::exit;
use std::ptr::addr_eq;
use text_io::read;
use crate::models;
use models::*;
use crate::handlers::{InputItem, ReadCommand, SaveUsersToJson};

pub fn OpenUserPanel(user: &mut User, restaurants: &mut Vec<Restaurant>, orders: &mut Vec<Order>) {
    let displayString = "here is user panel: \n\
    please select:\n\
    make new order: 1\n\
    display orders: 2\n\
    remove order: 3\n\
    pay order: 4\n\
    charge wallet: 5\n\
    display wallet: 6\n\
    exit: 9".to_string();
    println!("{displayString}");
    let mut command = ReadCommand();
    while command != 9 {
        match command {
            1 => {
                let mut selectedRestaurant = OpenRestaurantSelectionPanel(restaurants);
                let result = OpenRestaurantItemsSelectionPanel(&mut selectedRestaurant.unwrap(), user, orders);
                if let Err(e) = result {
                    println!("{e}");
                } else {
                    println!("order added successfully!");
                }
            }
            2 => {
                println!("here are your orders:");
                let result = user.DisplayOrders(orders);
                match result {
                    Some(data) => {
                        println!("{data}");
                    }
                    None => {
                        println!("no orders found!");
                    }
                }
            }
            3 => {
                let mut userOrders: &mut Vec<Order> = &mut Vec::new();
                for n in orders.iter_mut() {
                    if (n.userIndex == user.index) {
                        userOrders.push(n.clone());
                    }
                }
                if (userOrders.len() == 0) {
                    println!("no orders found!");
                } else {
                    println!("here are your orders:\nselect one to remove\nselect {} to exit", userOrders.len());
                }
                for (i, n) in userOrders.iter().enumerate() {
                    println!("{} {:?}", i, n);
                }
                let command = ReadCommand();
                if command != userOrders.len() as i32 {
                    orders.remove(userOrders[command as usize].orderIndex);
                    println!("order removed successfully!");
                } else {
                    println!("aborting...");
                }
            }
            4 => {
                if let Err(e) = OpenOrderPayPanel(user, orders) {
                    println!("{e}");
                }
            }
            5 => {
                println!("input amount you wanna charge your wallet:");
                let amount = ReadCommand();
                user.wallet += amount as u64;
                println!("wallet charged successfully!");
            }
            6 => {
                println!("the money you got is {} toomans", user.wallet)
            }
            _ => {
                println!("invalid command");
            }
        }
        println!("{displayString}");
        command = ReadCommand();
    }
}

pub fn OpenOrderPayPanel(user: &mut User, orders: &mut Vec<Order>) -> Result<(), String> {
    let mut userOrders = Vec::new();
    for n in orders {
        if (n.orderIndex == user.index) {
            userOrders.push(n);
        }
    }
    println!("select an order to pay or select {} to exit", userOrders.len());
    for (i, n) in userOrders.iter().enumerate() {
        match n.orderStatus {
            OrderStatus::inCart => {
                println!("{} {:?}", i, n);
            }
            _ => {}
        }
    }
    let command = ReadCommand() as usize;
    if (command == userOrders.len()) {
        return Err("order didnt paid!".to_string());
    }
    let order = &mut userOrders[command];
    let orderPrice = order.item.price * order.amount as u64;
    if ((orderPrice) > user.wallet) {
        return Err("not enough cash to pay this order!".to_string());
    }
    user.wallet -= orderPrice;
    order.orderStatus = OrderStatus::onWay;
    println!("order paid successfully. order is on way!");
    Ok(())
}
pub fn OpenRestaurantAdminPanel(restuarant: &mut Restaurant, orders: &mut Vec<Order>) {
    let displayString = "here is restaurant panel\n\
    please select:\n\
    add item into menu: 1\n\
    view items: 2\n\
    view orders: 3\n\
    change order status to finished: 4\n\
    exit: 9".to_string();
    println!("{displayString}");
    let mut command = ReadCommand();
    while command != 9 {
        match command {
            1 => {
                println!("please enter item name and price:");
                let item = InputItem();
                restuarant.items.push(item);
                println!("item added successfully")
            }
            2 => {
                println!("here are items available in your restaurant:");
                for (i, n) in restuarant.items.iter().enumerate() {
                    println!("{} {:?}", i, n);
                }
            }
            3 => {
                let mut restaurantOrders = Vec::new();
                for n in orders.iter() {
                    if (n.restaurantIndex == restuarant.index) {
                        restaurantOrders.push(n);
                    }
                }
                if restaurantOrders.len() > 0 {
                    println!("here are your orders:");
                    for (i, n) in restaurantOrders.iter().enumerate() {
                        println!("{} {:?}", i, n);
                    }
                } else {
                    println!("you have no orders");
                }
            }
            4 => {
                let result = restuarant.ChangeOrderStatusToFinished(orders);
                match result {
                    Err(e)=>{
                        println!("{e}");
                    }
                    Ok(())=>{
                        println!("order status changed successfully!");
                    }
                }
            }
            _ => {
                println!("invalid command!");
            }
        }
        println!("{displayString}");
        command = ReadCommand();
    }
}

pub fn OpenRestaurantSelectionPanel(restaurants: &mut Vec<Restaurant>) -> Option<&mut Restaurant> {
    if (restaurants.len() == 0) {
        println!("no restaurants available!");
        return None;
    }
    println!("here is the restaurants available. please select one:");
    for n in restaurants.iter() {
        println!("{} {}\n", n.index, n);
    }
    println!("enter {} to exit", restaurants.len());
    let command = ReadCommand();
    if (command == restaurants.len() as i32) {
        println!("aborting...");
        return None;
    }
    Some(&mut restaurants[command as usize])
}
pub fn OpenRestaurantItemsSelectionPanel(restaurant: &mut Restaurant, user: &mut User, orders: &mut Vec<Order>) -> Result<(), String> {
    if (restaurant.items.len() == 0) {
        return Err("restaurant has no item available!".to_string());
    }
    println!("here is items available for this restaurant. please select one:");
    for (i, n) in restaurant.items.iter().enumerate() {
        println!("{} {:?}", i, n);
    }
    println!("enter {} to exit", restaurant.items.len());
    let command = ReadCommand();
    if (command as usize == restaurant.items.len()) {
        return Err("aborting...".to_string());
    }
    println!("enter amount of your order:");
    let _amount = ReadCommand();
    let selectedItem: &Item = &restaurant.items[command as usize];
    let order = Order {
        userIndex: user.index,
        restaurantIndex: restaurant.index,
        item: selectedItem.clone(),
        orderStatus: OrderStatus::inCart,
        orderIndex: orders.len(),
        amount: _amount,
    };
    orders.push(order);
    Ok(())
}