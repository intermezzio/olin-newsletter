extern crate reqwest;
extern crate serde;
use serde::{Deserialize, Serialize};

pub async fn top_headlines(api_key: String) -> () { // Result<FnInfo, &'static str> {
	let base_url = String::from("https://newsapi.org/v2/top-headlines?country=us&apiKey=");
    let response = reqwest::get(base_url + &api_key)
    	.await
    	.unwrap();
    

    // let headlines_resp: NewsResponse = response.json().await.unwrap();

    println!("{:?}", response.text().await.unwrap());

}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct NewsResponse {
	status: String,
	totalResults: i32,
	articles: Vec<Article>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Article {
	source: ArticleSource,
	author: String,
	title: String,
	description: String,
	url: String,
	urlToImage: String,
	publishedAt: String,
	content: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ArticleSource {
	id: String,
	name: String
}