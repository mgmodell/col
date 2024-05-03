use serde::{Deserialize, Serialize};
use serde_json::Result;
use rand::Rng;
use std::collections::HashMap;
use gloo_storage::LocalStorage;
use gloo_storage::Storage;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

pub fn user_from_json(json: &str) -> Result<User> {
    let u: User = serde_json::from_str(json)?;
    Ok(u)
}

pub fn users_from_local_storage() -> HashMap<u128, User> {
    let users : HashMap<u128, User> = LocalStorage::get("users").unwrap_or_default();
    users
}

pub fn save_user_to_local_storage(user: User) -> HashMap<u128, User>{
    let mut users = users_from_local_storage();
    users.insert(user.id, user);
    LocalStorage::set("users", &users).unwrap();
    users
}