
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use rand::prelude::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Player {
    pub name: String,
    //health points
    pub weapon: String,

    pub hp: i32,
 
    //potentially add inventory capacity if time
}

impl Player {
    pub fn new(mut name: String, mut starter: String) -> Result<Self> {
        if name == "" {
            name = "Micah".to_string();
        }
        if starter == ""{
            starter = "Hammer".to_string();
        }
        

        Ok(Self {
            name: name,
            weapon: starter,
            hp: 100
            
        })
    }

    pub fn rename(self, name: String) -> Result<Self> {
       
        
        Ok(Self{
            name: name,
            weapon: self.weapon,
            hp: self.hp
        })
    }

}