extern crate reqwest;
extern crate serde;
use serde::{Deserialize, Serialize};

pub async fn rand_animal() -> Animal { // Result<FnInfo, &'static str> {
    let response = reqwest::get("https://zoo-animal-api.herokuapp.com/animals/rand")
    	.await
    	.unwrap();
    
    let animal: Animal = response.json().await.unwrap();

    animal.clone()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Animal {
	name: String,
	latin_name: String,
	animal_type: String,
	active_time: String,
	length_min: String,
	length_max: String,
	weight_min: String,
	weight_max: String,
	lifespan: String,
	habitat: String,
	diet: String,
	geo_range: String,
	image_link: String,
	id: i32,
}