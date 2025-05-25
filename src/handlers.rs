#![allow(non_snake_case)]
use std::io;
use std::io::{stdin, stdout, Write};
use std::ops::{Add, AddAssign};
use std::ptr::read;
use crate::models;
use models::*;
use text_io;
use text_io::read;

impl User {
    pub fn Login(self, users: &mut Vec<User>) -> Result<&mut User, String> {
        for n in users {
            if (self.username == n.username) {
                if (self.password == n.password) {
                    return Ok(n);
                }
            }
        }
        Err("username or password is wrong".to_string())
    }
    pub fn Register(mut self, users: &mut Vec<User>) -> Result<(), String> {
        for n in users.iter() {
            if self.username == n.username {
                return Err("user already exist!".to_string());
            }
        }
        self.index = users.len();
        users.push(self);
        return Ok(());
    }
    pub fn NewOrder(&mut self, _restaurant: &mut Restaurant, mut order: Order) {
        order.restaurantIndex = _restaurant.index;
        order.userIndex = self.index;
        self.orders.push(order.clone());
        _restaurant.orders.push(order.clone());
    }
    pub fn PayOrder(&mut self, order: &mut Order) {
        self.wallet -= order.item.price;
        order.orderStatus = OrderStatus::onWay;
    }

    pub fn DisplayOrders(&self) -> Option<String> {
        if (self.orders.len() > 0) {
            let mut result = String::new();
            for (i, n) in self.orders.iter().enumerate() {
                let string = format!("order: {}\n content: {:?}", i, n);
                result.add_assign(string.as_str())
            }
            return Some(result);
        }
        None
    }
}

impl Restaurant {
    pub fn Login(self, restaurants: &mut Vec<Restaurant>) -> Result<&mut Restaurant, String> {
        for n in restaurants.iter_mut() {
            if (n.username == self.username) {
                return Ok(n);
            }
        }
        Err("No such user exists".to_string())
    }
    pub fn Register(mut self, restaurants: &mut Vec<Restaurant>) -> Result<(), String> {
        for n in restaurants.iter() {
            if (n.username == self.username) {
                return Err("Restaurant already exists!".to_string());
            }
        }
        self.index = restaurants.len();
        restaurants.push(self);
        Ok(())
    }
    /*pub fn InputOrder()->Order{
        let item=InputItem();
        let order=Order{
            restaurantIndex:0,
            userIndex:0,
            orderStatus:OrderStatus::inCart,
            item:item,
        };
        order
    }
    pub fn InputItem()->Item{

    }
*/
}
impl Clone for User {
    fn clone(&self) -> Self {
        User {
            username: self.username.to_string(),
            password: self.password.to_string(),
            orders: self.orders.clone(),
            index: self.index,
            wallet: self.wallet.clone(),
        }
    }
}

pub fn InputUser() -> User {
    let username = read!();
    let password = read!();
    let mut user = User::default();
    user.username = username;
    user.password = password;
    user
}
pub fn InputRestaurant() -> Restaurant {
    let username = read!();
    let password = read!();
    let mut restaurant = Restaurant::default();
    restaurant.username = username;
    restaurant.password = password;
    restaurant
}

pub fn InputItem()->Item{
    let name=read!();
    let price=read!();
    let item=Item{
        name:name,
        price:price,
    };
    item
}
pub fn ReadCommand() -> i32 {
    let stdin = io::stdin();

    loop {
        print!("Enter a number: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read input");
        if input.trim().is_empty() {
            continue;
        }

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }
}