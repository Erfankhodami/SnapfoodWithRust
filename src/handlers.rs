use std::ops::{Add, AddAssign};
use crate::models;
use models::*;
use text_io;
use text_io::read;

impl User {
    pub fn Login(&mut self, users: &Vec<User>) -> Result<&mut User, String> {
        for n in users.iter() {
            if (self.username == n.username) {
                if (self.password == n.password) {
                    return Ok(self);
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
    pub fn NewOrder(&mut self, _restaurant: &mut Restaurant, mut order:Order) {
        order.restaurantIndex=_restaurant.index;
        order.userIndex=self.index;
        self.orders.push(order.clone());
        _restaurant.orders.push(order.clone());
    }
    pub fn PayOrder(&mut self, order: &mut Order) {
        self.wallet -= order.item.price;
        order.orderStatus = OrderStatus::onWay;
    }

    pub fn DisplayOrders(& self)->Option<String> {
        if(self.orders.len()>0){
            let mut result=String::new();
            for (i,n) in self.orders.iter().enumerate() {
                let string=format!("order: {}\n content: {:?}",i,n);
                result.add_assign(string.as_str())
            }
            return Some(result)
        }
        None
    }
}

impl Restaurant {
    pub fn Login(&self, restaurants: &Vec<Restaurant>) -> Result<&Restaurant, String> {
        for n in restaurants.iter() {
            if (n.username == self.username) {
                return Ok(self);
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


pub fn InputUser()->User{
    let username=read!();
    let password=read!();
    let mut user=User::default();
    user.username=username;
    user.password=password;
    user
}