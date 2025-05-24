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
use crate::Panels::OpenUserPanel;

fn main() {
    let mut restaurants:Vec<Restaurant>=Vec::new();
    restaurants.push(Restaurant::default());
    restaurants.push(Restaurant::default());
    let mut users: Vec<User> = Vec::new();
    let mut loggedInUser: Result<&mut User, String>;
    println!("here is main panel:\nplease select:\nregister new user: 1\nlogin user: 2\nexit: 9");
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
            _ => {
                println!("invalid command!");
            }
        };
        println!("here is main panel:\nplease select:\nregister new user: 1\nlogin user: 2\nexit: 9");
        command = ReadCommand();
    }
}


