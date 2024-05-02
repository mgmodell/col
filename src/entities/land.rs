use crate::entities::user::User;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use strum::VariantArray;
use rand::Rng;

pub mod land;

#[derive(VariantArray, Serialize, Deserialize)]
pub enum LandType {
    Desert,
    Forest,
    Grassland,
    Hills,
    Jungle,
    Mountains,
    Swamp,
    Tundra,
    Water,
    Other,
}

#[derive(VariantArray, Serialize, Deserialize)]
pub enum UpgradeType {
    Volcano,
    Quicksand,
    Sinkhole,
    Drought,
    Freshwater,
    IceAge,

}

#[derive(Serialize, Deserialize)]
pub enum LandEvent {
    Visited( User, DateTime<Utc>),
    Seeded( UpgradeType, DateTime<Utc>),
    Transferred( User, DateTime<Utc>),
}

#[derive(Serialize, Deserialize)]
pub struct Land {
    pub id: u128,
    pub land_type: LandType,
    pub history: Vec<LandEvent>,
}

impl Land {
    fn next_id() -> u128 {
        let mut rng = rand::thread_rng();
        rng.gen::<u128>()
    }

    pub fn new(user: User) -> Self {
        Self {
            id,
            land_type: LandType::VARIANTS.choose(rand::thread_rng()),
            history: vec![
                LandEvent::Transferred(user, Utc::now()),
            ],
        }
    }

    pub fn visit(&mut self, user: User, time: DateTime<Utc>) {
        self.history.push(LandEvent::Visited(user, time));
    }

    pub fn seed(&mut self, upgrade_type: UpgradeType, time: DateTime<Utc>) {
        self.history.push(LandEvent::Seeded(UpgradeType::VARIANTS.choose(rand::thread_rng()), time));
    }

    pub fn transfer(&mut self, user: User, time: DateTime<Utc>) {
        self.history.push(LandEvent::Transferred(user, time));
    }

    pub fn to_json(&self) -> Result<String> {
        let j = serde_json::to_string(&self);
        j
    }

}

pub fn generate_lands (user: User, count: u32) -> Vec<Land> {
    let mut lands = Vec::new();
    for _ in 0..count {
        lands.push(Land::new(user));
    }
    lands
}

pub fn lands_from_json(json: &str) -> Result<Vec<Land>> {
    let l: Vec<Land> = serde_json::from_str(json)?;
    Ok(l)
}