use crate::models;
use models::*;

impl Manager{
    pub fn Login(& mut self, username: String, password: String, mode: AccountMode) -> Result<(), String> {
        match mode {
            AccountMode::User => {
                for (i,n) in self.users.iter().enumerate() {
                    if (username == n.username) {
                        if (password == n.password) {
                            self.loggedInUserIndex= i;
                            return Ok(());
                        }
                    }
                }
                return Err("user not found!".to_string());
            }
            AccountMode::Restaurant => {
                for (i,n) in self.restaurants.iter().enumerate() {
                    if (username == n.username) {
                        if (password == n.password) {
                            self.loggedInRestaurantIndex=i;
                            return Ok(());
                        }
                    }
                }
                return Err("user not found!".to_string());
            }
        }
    }
    pub fn Register(&mut self, username: String, password: String, mode: AccountMode) -> Result<(), String> {
        for n in self.users.iter() {
            if username == n.username {
                return Err("user already exist!".to_string());
            }
        }
        self.users.push(User {
            username: username,
            password: password,
            orders: Vec::new(),
            wallet:0,
        });
        return Ok(());
    }
}

impl User{
    fn NewOrder(&mut self,_userIndex:usize,_restaurantIndex:usize,_item: Item,manager: Manager){
        let order=Order{
            restaurantIndex:_restaurantIndex,
            userIndex:_userIndex,
            item:_item,
            orderStatus:OrderStatus::inCart
        };
        self.orders.push(order.clone());
        manager.restaurants[_restaurantIndex].orders.push(order.clone());
    }
    fn PayOrder(&mut self,order:&mut Order){
        self.wallet-=order.item.price;
        order.orderStatus=OrderStatus::onWay;
    }

    fn DisplayOrders(&mut self){

    }
}