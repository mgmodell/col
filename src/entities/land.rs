use crate::entities::user::User;
use strum::VariantArray;
use rand;

pub mod land;

#[derive(VariantArray)]
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

#[derive(VariantArray)]
pub enum UpgradeType {
    Volcano,
    Quicksand,
    Sinkhole,
    Drought,
    Freshwater,
    IceAge,

}

pub enum LandEvent {
    Visited( User, DateTime<Utc>),
    Seeded( UpgradeType, DateTime<Utc>),
    Transferred( User, DateTime<Utc>),
}

pub struct Land {
    pub id: u128,
    pub land_type: LandType,
    pub history: Vec<LandEvent>,
}

impl Land {
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

}

pub fn generate_lands (user: User) -> Vec<Land> {
    let mut lands = Vec::new();
    for _ in 0..10 {
        lands.push(Land::new(user));
    }
    lands
}