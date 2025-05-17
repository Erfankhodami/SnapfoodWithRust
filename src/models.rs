#![allow(non_snake_case)]
#![allow(non_camel_case_types)]


pub enum AccountMode{
    User,
    Restaurant,
}
#[derive(Clone)]
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
#[derive(Default,Clone)]
pub struct Order{
    pub restaurantIndex: usize,
    pub userIndex: usize,
    pub item: Item,
    pub orderStatus:OrderStatus,
}


#[derive(Default,Clone)]
pub struct Item{
    pub name:String,
    pub price:u64,
}

#[derive(Default)]
pub struct User{
    pub username:String,
    pub password:String,
    pub orders: Vec<Order>,
    pub wallet: u64,
}
#[derive(Default)]
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