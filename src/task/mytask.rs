use crate::{Serialize,Deserialize};
#[derive(Debug,Serialize,Deserialize)]
pub struct People {
   pub name: String,
    pub age: u32,
   pub  isactive: bool,
    pub address: Address,
}
#[derive(Debug,Deserialize,Serialize)]
pub struct Address {
    pub city: String,
    pub home: String,
   pub country: String,
}