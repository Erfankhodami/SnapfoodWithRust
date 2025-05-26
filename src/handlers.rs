#![allow(non_snake_case)]

use std::f32::consts::E;
use std::fmt::{Display, Formatter};
use std::io;
use std::io::{stdin, stdout, Write};
use std::ops::{Add, AddAssign};
use std::ptr::read;
use crate::models;
use models::*;
use text_io;
use text_io::read;
use serde::{Serialize, Deserialize};
use std::fs;
use std::fs::File;
use std::path::Path;

impl User {
    pub fn Login(self, users: &mut Vec<User>) -> Result<&mut User, String> {
        for n in users {
            if (self.username == n.username) {
                if (self.password == n.password) {
                    if(n.isBanned){
                        return Err("user is banned!".to_string());
                    }
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
    pub fn DisplayOrders(&self, orders: &mut Vec<Order>) -> Option<String> {
        if (orders.len() > 0) {
            let mut result = String::new();
            for (i, n) in orders.iter().enumerate() {
                if (n.userIndex == self.index) {
                    let string = format!("order: {}\n content: {:?}", i, n);
                    result.add_assign(string.as_str())
                }
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
                if(n.isBanned){
                    return Err("restaurant is banned!".to_string());
                }
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
    pub fn ChangeOrderStatusToFinished(&self,orders:&mut Vec<Order>)->Result<(),String>{
        let mut restaurantOrders = Vec::new();
        for n in orders.iter_mut() {
            match n.orderStatus {
                OrderStatus::Done => {
                    continue;
                }
                _ => {}
            }
            if (n.restaurantIndex == self.index) {
                restaurantOrders.push(n);
            }
        }
        if (restaurantOrders.len() == 0) {
            return Err("you have no in-progress orders!".to_string())
        }
        println!("here are your orders:\nplease select one\nor select {} to exit", restaurantOrders.len());
        for n in restaurantOrders.iter() {
            println!("{} {:?}", n.orderIndex, n);
        }
        let command = ReadCommand();
        if (command == restaurantOrders.len() as i32) {
            return Err("aborting...".to_string());
        }
        restaurantOrders[command as usize].orderStatus = OrderStatus::Done;
        Ok(())
    }
}
impl Display for Restaurant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{},{:?}",self.username,self.items)
    }
}
impl Clone for User {
    fn clone(&self) -> Self {
        User {
            username: self.username.to_string(),
            password: self.password.to_string(),
            index: self.index,
            wallet: self.wallet.clone(),
            isBanned:self.isBanned,
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

pub fn InputItem() -> Item {
    let name = read!();
    let price = read!();
    let item = Item {
        name,
        price,
    };
    item
}
pub fn ReadCommand() -> i32 {
    let stdin = io::stdin();
    print!("Enter a number: ");
    loop {
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

pub fn SaveUsersToJson(users: &Vec<User>) {
    let path = Path::new("data/user.json");
    let gonnaSaveData = serde_json::to_string_pretty(users).unwrap();
    if (!path.exists()) {
        if let Err(e) = File::create(path) {
            println!("{e}");
        }
    }
    let result = fs::write(path, gonnaSaveData);
    if let Err(e) = result {
        println!("error happened in saving files to json!: {e}");
        return;
    }
    println!("file saved successfully");
}
pub fn LoadUserFromJson() -> Result<Vec<User>, String> {
    let path = Path::new("data/user.json");
    let gonnaLoadData = fs::read_to_string(path);
    if let Err(e) = gonnaLoadData {
        return Err(e.to_string());
    }
    let users = serde_json::from_str(gonnaLoadData.unwrap().as_str());
    if let Err(e) = users {
        return Err(e.to_string());
    }
    return Ok(users.unwrap());
}
pub fn SaveRestaurantsToJson(restaurants: &Vec<Restaurant>) {
    let path = Path::new("data/restaurants.json");
    let gonnaSaveData = serde_json::to_string_pretty(restaurants).unwrap();
    if (!path.exists()) {
        if let Err(e) = File::create(path) {
            println!("{e}");
        }
    }
    let result = fs::write(path, gonnaSaveData);
    if let Err(e) = result {
        println!("error happened in saving files to json!: {e}");
        return;
    }
    println!("file saved successfully");
}
pub fn LoadRestaurantFromJson() -> Result<Vec<Restaurant>, String> {
    let path = Path::new("data/restaurants.json");
    let gonnaLoadData = fs::read_to_string(path);
    if let Err(e) = gonnaLoadData {
        return Err(e.to_string());
    }
    let restaurants = serde_json::from_str(gonnaLoadData.unwrap().as_str());
    if let Err(e) = restaurants {
        return Err(e.to_string());
    }
    return Ok(restaurants.unwrap());
}
pub fn SaveOrdersToJson(orders:&Vec<Order>) {
    let path = Path::new("data/orders.json");
    let gonnaSaveData = serde_json::to_string_pretty(orders).unwrap();
    if (!path.exists()) {
        if let Err(e) = File::create(path) {
            println!("{e}");
        }
    }
    let result = fs::write(path, gonnaSaveData);
    if let Err(e) = result {
        println!("error happened in saving files to json!: {e}");
        return;
    }
    println!("file saved successfully");
}
pub fn LoadOredersFromJson()->Result<Vec<Order>,String>{
    let path = Path::new("data/orders.json");
    let gonnaLoadData = fs::read_to_string(path);
    if let Err(e) = gonnaLoadData {
        return Err(e.to_string());
    }
    let orders = serde_json::from_str(gonnaLoadData.unwrap().as_str());
    if let Err(e) = orders {
        return Err(e.to_string());
    }
    Ok(orders.unwrap())
}

