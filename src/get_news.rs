extern crate reqwest;
extern crate serde;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};

pub async fn top_headlines(api_key: String) -> NewsResponse { // Result<FnInfo, &'static str> {
	let base_url = "http://newsapi.org/v2/top-headlines?country=us";
	let mut headers = HeaderMap::new();
	headers.insert(USER_AGENT, HeaderValue::from_static("Sample Rust Program"));
	let api_key_header = HeaderName::from_lowercase(b"x-api-key").unwrap();
	headers.insert(api_key_header, api_key.parse().unwrap());
    let client = reqwest::Client::builder()
		.default_headers(headers)
		.build()
		.unwrap();
	let response = client.get(base_url)
		.send()
    	.await
    	.unwrap();
    
    let headlines_resp: NewsResponse = response.json().await.unwrap();

    // TODO: actually parse this chunky response
    headlines_resp
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewsResponse {
	status: String,
	totalResults: i32,
	articles: Vec<Article>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Article {
	source: Option<ArticleSource>,
	author: Option<String>,
	title: Option<String>,
	description: Option<String>,
	url: Option<String>,
	urlToImage: Option<String>,
	publishedAt: Option<String>,
	content: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ArticleSource {
	id: Option<String>,
	name: Option<String>
}