#![allow(non_snake_case)]
#![allow(non_camel_case_types)]


pub enum AccountMode{
    User,
    Restaurant,
}
pub struct Order{
    pub restaurant: Restaurant,
    pub user: User,
}
pub struct Item{
    pub name:String,
    pub price:u32,
}

pub struct User{
    pub username:String,
    pub password:String,
    pub orders: Vec<Order>,
}
pub struct Restaurant{
    pub username:String,
    pub password:String,
    pub items:Vec<Item>,
    pub orders: Vec<Order>,
}

#[derive(Default)]
pub struct Manager{
    pub users:Vec<User>,
    pub restaurants: Vec<Restaurant>,
    pub loggedInUserIndex:usize,
    pub loggedInRestaurantIndex:usize,
}