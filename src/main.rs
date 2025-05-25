#![allow(non_snake_case)]
mod models;
mod handlers;
mod Panels;

use std::fmt::format;
use std::ptr::read;
use std::rc::Rc;
use std::vec;
use models::*;
use handlers::*;
use text_io;
use text_io::read;
use crate::Panels::{OpenRestaurantAdminPanel, OpenUserPanel};

fn main() {
    let mut users: Vec<User> = Vec::new();
    let mut loggedInUser: Result<&mut User, String>;
    let mut restaurants:Vec<Restaurant>=Vec::new();
    let mut loggedInRestaurant:Result<&mut Restaurant,String>;

    match LoadUserFromJson() {
        Ok(_users)=>{
            users=_users;
        }
        Err(e)=>{
            println!("{e}");
        }
    }
    let mainPanelDisplayString=
        "here is main panel:\nplease select:\nregister new user: 1\nlogin user: 2\nregister new restaurant: 3\nrestaurant login: 4\nexit: 9".to_string();
    println!("{mainPanelDisplayString}");
    let mut command = ReadCommand();

    while command != 9 {
        match command {
            1 => {
                println!("input username and password");
                let user = InputUser();
                let result = user.Register(&mut users);
                match result {
                    Err(e) => {
                        println!("{e}");
                    }
                    Ok(()) => {
                        println!("registered successfully!")
                    }
                }
            }
            2 => {
                println!("input username and password");
                let user = InputUser();
                loggedInUser = user.Login(&mut users);
                match loggedInUser {
                    Err(e) => {
                        println!("{e}");
                    }
                    _ => {
                        println!("logged in successfully!");
                        OpenUserPanel(&mut loggedInUser.unwrap(), &mut restaurants)
                    }
                }
            }
            3=>{
                println!("input the yourRestaurant name and password: ");
                let restaurant=InputRestaurant();
                let result=restaurant.Register(&mut restaurants);
                match result {
                    Ok(())=>{
                        println!("restaurant registered successfully!");
                    }
                    Err(e)=>{
                        println!("{e}");
                    }
                }
            }
            4=>{
                println!("input restaurant name and password:");
                let restaurant=InputRestaurant();
                loggedInRestaurant=restaurant.Login(&mut restaurants);
                match loggedInRestaurant {
                    Err(e)=>{
                        println!("{e}");
                    }
                    _=>{
                        println!("logged in successfully!");
                        OpenRestaurantAdminPanel(&mut loggedInRestaurant.unwrap());
                    }
                }
            }
            _ => {
                println!("invalid command!");
            }
        };
        SaveUsersToJson(&users);
        println!("{mainPanelDisplayString}");
        command = ReadCommand();
    }
}


