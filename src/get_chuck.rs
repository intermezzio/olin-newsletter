extern crate reqwest;
extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Quote {
	categories: Vec<String>,
	created_at: String,
	icon_url: String,
	id: String,
	updated_at: String,
	url: String,
    value: String,
}

pub async fn chuck_norris_quote() -> String { // Result<FnInfo, &'static str> {
    let response = reqwest::get("https://api.chucknorris.io/jokes/random?category=dev")
    	.await
    	.unwrap();
    
    let chuck_quote = response.json::<Quote>().await.unwrap();

    chuck_quote.value
}

