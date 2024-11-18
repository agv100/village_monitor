use axum::{
   routing::get,
   Router,
   extract::{Path, Query, Json, State},
};
use serde::{Deserialize};
use chrono::{DateTime,  Local};
use influx_db_client::{Client, Point, Points, Value, Precision};
use url::Url;


#[tokio::main]
async fn main() {
//    println!("Hello, world!");
//  Init connection to influxdb 
//    let influx_url = Url::parse("http://localhost:8086").unwrap();
//    let influx_client = Client::new(influx_url, "Sensors111");


//  Init application on 3000 port, with /temp?location=...&temp=...&humidity is GET string using
//  which ESP32s passing values to database

    let app=Router::new()
        .route("/temp", get(get_temp)); 
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

async fn get_temp(DHT22: Query<dht22> )  {
   let MyDHT22: dht22 = DHT22.0;
   let influx_url = Url::parse("http://localhost:8086").unwrap();
   let influx_client = Client::new(influx_url, "Sensors111");

   let measurement_time = Local::now();
   println!("{measurement_time} Location:{} Temp:{} Humidity:{}",MyDHT22.location, MyDHT22.temp, MyDHT22.humidity);
   let mypoint = Point::new("DHT22s")
          .add_field("location", Value::String(MyDHT22.location))
          .add_field("temp",Value::String(MyDHT22.temp))
          .add_field("humidity",Value::String(MyDHT22.humidity));

   influx_client.write_point(mypoint,  Some(Precision::Seconds), None ).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct dht22 {
        location: String,
        temp: String,
        humidity: String  
            
}
