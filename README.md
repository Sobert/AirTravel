# AirPlane exercise
## Models
### Airport
```rust
enum Airport {
    DTW,
    JFK,
    CDG,
    LAD
}
```
### Plane
```rust
struct Plane {
    name: String,
    total_seats: i32,
}
```
### Flight
```rust
struct Flight {
    code: String,
    departure: Airport,
    arrival: Airport,
    base_price: i32,
    plane: Plane,
    seats_booked: i32,
}
```
### Ticket
```rust
struct Ticket {
    code: String,
    flight: Flight,
    date: String,
    payed_price: i32,
    customer_name: String,
    customer_nationality: String,
    options: Option<Vec<FlightOptions>>,
    booking_source: String,
}
```
### FlightOptions
```rust
struct FlightOptions {
    option_type: OptionType,
    price: i32,
}
```
### OptionType
```rust
enum OptionType {
    BonusLuggage,
    FirstClass,
    ChampagneOnBoard,
    LoungeAccess,
}
```
