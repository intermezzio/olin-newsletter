extern crate reqwest;
extern crate serde;
use serde::{Deserialize, Serialize};

pub async fn download() -> Quote { // Result<FnInfo, &'static str> {
    let response = reqwest::get("https://api.chucknorris.io/jokes/random?category=dev")
    	.await
    	.unwrap();
    
    let chuck_quote: Quote = response.json().await.unwrap();

    chuck_quote
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
	categories: Vec<String>,
	created_at: String,
	icon_url: String,
	id: String,
	updated_at: String,
	url: String,
    value: String,
}