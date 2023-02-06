use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InstantiateMsg { // only the admin is able to instantiate the contract
    pub admins: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ExecuteMsg { // The admin can also add additional admins or remove themself as admin
    AddMembers { admins: Vec<String> },
    Leave {},
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GreetResp {
   pub message: String,
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum QueryMsg {
    Greet {}, // Without "{}" the JSON would serialize to just a string type. It is a good habit to always add the {} to serde serializable empty enum variants - for better JSON representation. 
    AdminsList {},
}

