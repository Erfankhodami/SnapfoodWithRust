mod models;
mod handlers;
mod moudele;

use std::fmt::format;
use std::vec;
use models::*;
use handlers::*;

fn main() {
    let mut manager = Manager::default();
    let result=manager.Register("name".to_string(),"pass".to_string(),AccountMode::User);
    match result {
        Ok(())=>{}
        Err(e)=>{
            println!("error:{}",e);
        }
    }
    let result=manager.Login("name".to_string(),"pass".to_string(),AccountMode::User);
    println!("{}",manager.loggedInUserIndex);
    println!("{}",manager.users.len());
    println!("{}",manager.users[manager.loggedInRestaurantIndex].password);
}

