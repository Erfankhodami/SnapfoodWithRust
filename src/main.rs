mod models;
mod handlers;
use std::fmt::format;
use std::rc::Rc;
use std::vec;
use models::*;
use handlers::*;

fn main() {
    println!("please select:\nmake new user: 1\nlogin user: 2");
    let mut users: Vec<User> = Vec::new();
    println!("making a new user input your data:\n");
    let mut user1 = InputUser();

    match user1.Register(&mut users) {
        Ok(())=>{}
        Err(e)=>{
            println!("{e}");
        }
    }



    let mut gonnaLoginUser = InputUser();
    gonnaLoginUser.password = "pass".to_string();
    let loggedInUser:&mut User = gonnaLoginUser.Login(&users).unwrap();
    println!("{}", loggedInUser.password);


    let mut resturatnts: Vec<Restaurant> = Vec::new();
    let mut rest1 = Restaurant::default();
    rest1.username = "user".to_string();
    rest1.password = "pass".to_string();
    rest1.Register(&mut resturatnts).unwrap();
    let mut gonnaLoginRest = Restaurant::default();
    gonnaLoginRest.username = "user".to_string();
    gonnaLoginRest.password = "pass".to_string();
    let loggedInRest = gonnaLoginRest.Login(&resturatnts).unwrap();
    println!("{}", loggedInRest.password);


    let mut order =Order::default();
    let mut item =Item::default();
    item.name="pizza".to_string();
    item.price=100;
    order.item=item;
    loggedInUser.NewOrder(&mut resturatnts[0],order);


    let mut order =Order::default();
    let mut item =Item::default();
    item.name="piaszza".to_string();
    item.price=1005;
    order.item=item;
    loggedInUser.NewOrder(&mut resturatnts[0],order);
    println!("{}",loggedInUser.DisplayOrders().unwrap());

}

