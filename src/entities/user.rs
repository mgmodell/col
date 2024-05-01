use serde::{Deserialize, Serialize};
use serde_json::{json, Result};
//use crate::entities::user; // Add the missing user module
use rand::Rng;


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u128,
    pub name: String,
    //pub email: String,
    //pub password: String,
    //pub created_at: DateTime<Utc>,
    //pub updated_at: DateTime<Utc>,
}

impl User {
    fn next_id() -> u128 {
        let mut rng = rand::thread_rng();
        rng.gen::<u128>()
    }

    pub fn new(name: String) -> Self {
        Self {
            id: User::next_id(),
            name,
            //email,
            //password,
            //created_at: Utc::now(),
            //updated_at: Utc::now(),
        }
    }

    pub fn to_json(&self) -> Result<String> {
        let j = serde_json::to_string(&self);
        j
    }

}

pub fn from_json(json: &str) -> Result<User> {
    let u: User = serde_json::from_str(json)?;
    Ok(u)
}