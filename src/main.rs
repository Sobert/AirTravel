#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use std::env;

use rocket::config::{Config, Environment};

use std::io::Read;

use rocket::{Request, Data, Outcome::*};
use rocket::data::{self, FromDataSimple};
use rocket::http::{Status, ContentType};

use rocket::State;
use std::sync::Mutex;

mod model;
use model::*;


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
    app.mount("/", routes![get_flight_list, get_flight_options, post_ticket, get_tickets])
    .manage(SharedData {
        flights: Mutex::new(fill_flights()),
        options: Mutex::new(fill_flight_options()),
        tickets: Mutex::new(Vec::new()),
    })
    .launch();
}

fn fill_flights() -> Vec<Flight> {
    vec![
        Flight {
            code: "AF345".to_string(),
            departure: Airport::DTW,
            arrival: Airport::JFK,
            base_price: 300,
            plane: Plane {
                name: "AIRBUS350".to_string(),
                total_seats: 200,
            },
            seats_booked: 0,
        },
        Flight {
            code: "AF346".to_string(),
            departure: Airport::DTW,
            arrival: Airport::CDG,
            base_price: 700,
            plane: Plane {
                name: "AIRBUS750".to_string(),
                total_seats: 700,
            },
            seats_booked: 0,
        },
        Flight {
            code: "AF347".to_string(),
            departure: Airport::CDG,
            arrival: Airport::JFK,
            base_price: 1000,
            plane: Plane {
                name: "AIRBUS950".to_string(),
                total_seats: 1000,
            },
            seats_booked: 0,
        },
        Flight {
            code: "AF348".to_string(),
            departure: Airport::CDG,
            arrival: Airport::LAD,
            base_price: 300,
            plane: Plane {
                name: "AIRBUS450".to_string(),
                total_seats: 400,
            },
            seats_booked: 0,
        },
    ]
}

fn fill_flight_options() -> Vec<FlightOptions> {
    vec![
        FlightOptions {
            option_type: OptionType::BonusLuggage,
            price: 100,
        },
        FlightOptions {
            option_type: OptionType::ChampagneOnBoard,
            price: 150,
        }
    ]
}


#[get("/flights/<date>")]
fn get_flight_list(date: String, shared: State<SharedData>) -> String {
    let shared_data: &SharedData = shared.inner();
    serde_json::to_string(&shared_data.flights).unwrap()
}

#[get("/available_options/<flight>")]
fn get_flight_options(flight: String, shared: State<SharedData>) -> String {
    let shared_data: &SharedData = shared.inner();
    serde_json::to_string(&shared_data.options).unwrap()
}

#[get("/tickets")]
fn get_tickets(shared: State<SharedData>) -> String {
    let shared_data: &SharedData = shared.inner();
    serde_json::to_string(&shared_data.tickets).unwrap()
}

#[post("/book", format = "application/json", data = "<ticket>")]
fn post_ticket(ticket: Ticket, shared: State<SharedData>) -> String { 
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
    let shared_data: &SharedData = shared.inner();
    shared_data.tickets.lock().unwrap().push(success_ticket);
    serde_json::to_string("success").unwrap()
}

//structs

struct SharedData {
    flights: Mutex<Vec<Flight>>,
    options: Mutex<Vec<FlightOptions>>,
    tickets: Mutex<Vec<Ticket>>,
}


// Always use a limit to prevent DoS attacks.
const LIMIT: u64 = 100000;

impl FromDataSimple for Ticket {
    type Error = String;
    fn from_data(_req: &Request, data: Data) -> data::Outcome<Self, String> {
        // Read the data into a String.
        let mut string = String::new();
        if let Err(e) = data.open().take(LIMIT).read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }
        //Deserialize
        match serde_json::from_str(&string) {
            Ok(t) => Success(t),
            Err(e) =>  { 
                println!("{:#?}", e);
                Failure((Status::UnprocessableEntity, format!("{:?}", e)))
            }
        }
    }
}