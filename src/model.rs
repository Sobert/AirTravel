use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Airport {
    DTW,
    JFK,
    CDG,
    LAD
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Plane {
    pub name: String,
    pub total_seats: i32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Flight {
    pub code: String,
    pub departure: Airport,
    pub arrival: Airport,
    pub base_price: i32,
    pub plane: Plane,
    pub seats_booked: i32,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct FlightOptions {
    pub option_type: OptionType,
    pub price: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OptionType {
    BonusLuggage,
    FirstClass,
    ChampagneOnBoard,
    LoungeAccess,
}