
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::cell::RefCell;
use serde::{Serialize,Deserialize};
#[derive(Serialize,Deserialize,Clone,Debug)]
pub enum OrderStatus{
    inCart,
    onWay,
    Done,
    None,
}
impl Default for OrderStatus{
    fn default() -> Self {
        OrderStatus::None
    }
}
#[derive(Serialize,Deserialize,Default,Clone,Debug)]
pub struct Order{
    pub restaurantIndex: usize,
    pub userIndex: usize,
    pub item: Item,
    pub orderStatus:OrderStatus,
}


#[derive(Serialize,Deserialize,Default, Clone, Debug)]
pub struct Item{
    pub name:String,
    pub price:u64,
}


#[derive(Serialize,Deserialize,Default,Debug)]
pub struct User{
    pub username:String,
    pub password:String,
    pub orders: Vec<Order>,
    pub index:usize,
    pub wallet: u64,
}

#[derive(Serialize,Deserialize,Default,Debug)]
pub struct Restaurant{
    pub username:String,
    pub password:String,
    pub items:Vec<Item>,
    pub orders: Vec<Order>,
    pub index:usize,
}
