use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Airport {
    DTW,
    JFK,
    CDG,
    LAD
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Plane {
    pub name: String,
    pub total_seats: i32,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flight {
    pub code: String,
    pub departure: Airport,
    pub arrival: Airport,
    pub base_price: i32,
    pub plane: Plane
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticket {
    pub code: Option<String>,
    pub flight: Flight,
    pub date: String,
    pub payed_price: i32,
    pub customer_name: String,
    pub customer_nationality: String,
    pub options: Option<Vec<FlightOptions>>,
    pub booking_source: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlightOptions {
    pub option_type: OptionType,
    pub price: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OptionType {
    BonusLuggage,
    FirstClass,
    ChampagneOnBoard,
    LoungeAccess,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlightAvailability {
    pub flight: Flight,
    pub availability: i32,
}