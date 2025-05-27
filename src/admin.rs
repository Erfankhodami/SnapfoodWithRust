use crate::handlers::ReadCommand;
use crate::models::{Restaurant, User};

pub fn OpenAdminPanel(users:&mut Vec<User>, restaurants:&mut Vec<Restaurant>){
    let displayString="here is admin panel:\nview users: 1\nview restaurants: 2\nban a user: 3\nban a restaurant: 4\
    \nunban a user: 5\
    \nunban a restaurant: 6\
    \nexit: 9".to_string();
    println!("{displayString}");
    loop {
        let mut command=ReadCommand();
        match command {
            1=>{
                for n in users.iter() {
                    println!("{:?}",n);
                }
            }
            2=>{
                for n in restaurants.iter() {
                    println!("{}",n);
                }
            }
            3=>{
                println!("select a user to ban or select {} to exit",users.len());
                for (i,n) in users.iter().enumerate() {
                    println!("{} {:?}",i,n);
                }
                let command=ReadCommand() as usize;
                if(command==users.len()){
                    println!("aborting...");
                    continue
                }
                users[command].isBanned=true;
                println!("user banned successfully!");
            }
            4=>{
                println!("select a restaurant to ban or select {} to exit",restaurants.len());
                for (i,n) in restaurants.iter().enumerate() {
                    println!("{} {}",i,n);
                }
                let command=ReadCommand() as usize;
                if(command==restaurants.len()){
                    println!("aborting...");
                    continue
                }
                restaurants[command].isBanned=true;
                println!("restaurant banned successfully!");

            }
            5=>{
                let mut bannedUsers=Vec::new();
                for n in users.iter() {
                    if(n.isBanned){
                        bannedUsers.push(n);
                    }
                }
                if(bannedUsers.len()==0){
                    println!("no user is banned here!");
                    continue
                }
                println!("select a banned user to unban or select {} to exit",bannedUsers.len());
                for (i,n) in bannedUsers.iter().enumerate() {
                    println!("{} {:?}",i,n);
                }
                let command=ReadCommand() as usize;
                if(command==users.len()){
                    println!("aborting...");
                    continue
                }
                users[command].isBanned=false;
                println!("user unbanned successfully!");
            }
            6=>{
                let mut bannedRestaurants=Vec::new();
                for n in restaurants.iter() {
                    if(n.isBanned){
                        bannedRestaurants.push(n);
                    }
                }
                if(bannedRestaurants.len()==0){
                    println!("no restaurant is banned here!");
                    continue
                }
                println!("select a banned restaurant to unban or select {} to exit",bannedRestaurants.len());
                for (i,n) in bannedRestaurants.iter().enumerate() {
                    println!("{} {}",i,n);
                }
                let command=ReadCommand() as usize;
                if(command==restaurants.len()){
                    println!("aborting...");
                    continue
                }
                restaurants[command].isBanned=false;
                println!("restaurant unbanned successfully!");

            }
            9=>{
                return;
            }
            _=>{
                println!("invalid command!");
            }
        }
        println!("{displayString}");
        command=ReadCommand();
    }
}