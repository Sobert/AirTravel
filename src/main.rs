#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use std::env;

use serde::{Deserialize, Serialize};

use rocket::config::{Config, Environment};

use std::io::Read;

use rocket::{Request, Data, Outcome, Outcome::*};
use rocket::data::{self, FromDataSimple};
use rocket::http::{Status, ContentType};

fn main() {
    let mut port: u16 = 8080;
    match env::var("PORT") {
        Ok(p) => {
            match p.parse::<u16>() {
                Ok(n) => {
                    port = n;
                }
                Err(_e) => {}
            };
        }
        Err(_e) => {}
    };

    let config = Config::build(Environment::Production)
        .port(port)
        .secret_key("jU8LWhvJZENBmAkC9z9ULQpp1j+vgZPXkpgviXXKq04=")
        .finalize()
        .unwrap();

    let app = rocket::custom(config);
    app.mount("/", routes![get_flight_list, get_flight_options, post_ticket])
    .launch();
}


#[get("/flights/<date>")]
fn get_flight_list(date: String) -> String {
    let flights = vec![
        Flight {
            code: "AF345".to_string(),
            departure: Airport::DTW,
            arrival: Airport::JFK,
            base_price: 300,
            plane: Plane {
                name: "AIRBUS350".to_string(),
                total_seats: 400,
            },
            seats_booked: 0,
        }
    ];
    serde_json::to_string(&flights).unwrap()
}

#[get("/available_options/<flight>")]
fn get_flight_options(flight: String) -> String {
    let flight_options = vec![
        FlightOptions {
            option_type: OptionType::BonusLuggage,
            price: 100,
        },
        FlightOptions {
            option_type: OptionType::ChampagneOnBoard,
            price: 150,
        }
    ];
    serde_json::to_string(&flight_options).unwrap()
}

#[post("/book", format = "application/json", data = "<ticket>")]
fn post_ticket(ticket: Ticket) -> String { 
    let success_ticket = Ticket {
        code: Some("Success".to_string()),
        flight: ticket.flight,
        date: ticket.date,
        payed_price: ticket.payed_price,
        customer_name: ticket.customer_name,
        customer_nationality: ticket.customer_nationality,
        options: ticket.options,
        booking_source: ticket.booking_source,
    };
    serde_json::to_string(&success_ticket).unwrap()
}

//models

#[derive(Serialize, Deserialize, Debug)]
enum Airport {
    DTW,
    JFK,
    CDG,
    LAD
}


#[derive(Serialize, Deserialize, Debug)]
struct Plane {
    name: String,
    total_seats: i32,
}


#[derive(Serialize, Deserialize, Debug)]
struct Flight {
    code: String,
    departure: Airport,
    arrival: Airport,
    base_price: i32,
    plane: Plane,
    seats_booked: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Ticket {
    code: Option<String>,
    flight: Flight,
    date: String,
    payed_price: i32,
    customer_name: String,
    customer_nationality: String,
    options: Option<Vec<FlightOptions>>,
    booking_source: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FlightOptions {
    option_type: OptionType,
    price: i32,
}

#[derive(Serialize, Deserialize, Debug)]
enum OptionType {
    BonusLuggage,
    FirstClass,
    ChampagneOnBoard,
    LoungeAccess,
}

// Always use a limit to prevent DoS attacks.
const LIMIT: u64 = 256;

impl FromDataSimple for Ticket {
    type Error = String;
    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        // Read the data into a String.
        let mut string = String::new();
        if let Err(e) = data.open().take(LIMIT).read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }
        //Deserialize
        match serde_json::from_str(&string) {
            Ok(t) => Success(t),
            Err(e) => Failure((Status::UnprocessableEntity, format!("{:?}", e)))
        }
    }
}