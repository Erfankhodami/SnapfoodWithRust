
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::cell::RefCell;

pub enum AccountMode{
    _User,
    _Restaurant,
}
#[derive(Clone,Debug)]
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
#[derive(Default,Clone,Debug)]
pub struct Order{
    pub restaurantIndex: usize,
    pub userIndex: usize,
    pub item: Item,
    pub orderStatus:OrderStatus,
}


#[derive(Default, Clone, Debug)]
pub struct Item{
    pub name:String,
    pub price:u64,
}


#[derive(Default)]
pub struct User{
    pub username:String,
    pub password:String,
    pub orders: Vec<Order>,
    pub index:usize,
    pub wallet: u64,
}

#[derive(Default)]
pub struct Restaurant{
    pub username:String,
    pub password:String,
    pub items:Vec<Item>,
    pub orders: Vec<Order>,
    pub index:usize,
}
