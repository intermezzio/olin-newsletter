extern crate reqwest;
extern crate serde;
use serde::{Deserialize, Serialize};

pub async fn download(api_key: String) -> WeatherData { // Result<FnInfo, &'static str> {
	println!("Hello World");

	WeatherData {}
}

#[derive(Debug, Clone)]
pub struct WeatherData {

}