# API 

Simple booking flight API

## Methods

### Get all Flights
```bash
GET /flights
```

JSON reply
```jsonc
[
  {
      "code": "<flight>",
      "departure" : "<airport code>",
      "arrival" : "<airport code>",
      "base_price": 800, //price
      "plane": {
          "name" : "<plane name>",
          "total_seats" : 200, //total number of seats
      }
  }
]
```


### Get Flights on date
```bash
GET /flights/<date>
```
__<date>__ is of format "dd-MM-yyyy" (e.g: GET /flights/20-12-2020)

JSON reply
```jsonc
[
  {
      "code": "<flight>",
      "departure" : "<airport code>",
      "arrival" : "<airport code>",
      "base_price": 800, //price
      "plane": {
          "name" : "<plane name>",
          "total_seats" : 200, //total number of seats
      },
      "availability": 0 //number of seats left
  }
]
```

### Get Available Options
```bash
GET /available_options/<flight>
```

JSON reply
```jsonc
[
  {
      "option_type": "<option name>",
      "price": 200 //price
  }
]

```

### Book Flight
```bash
POST /book
```

JSON payload

```jsonc
{
  "flight": {
      "code": "<flight>",
      "departure" : "<airport code>",
      "arrival" : "<airport code>",
      "base_price": 800, //price
      "plane": {
          "name" : "<plane name>",
          "total_seats" : 200, //total number of seats
      }
  },
    "date": "<flight date>", //dd-mm-yyyy
    "payed_price": 300, //price
    "customer_name":"<full name>",
    "customer_nationality": "<nationality>",
    "options": [
      {
          "option_type": "<option name>",
          "price": 200 //price
      }]
    "booking_source": "<reference source>"
}
```
