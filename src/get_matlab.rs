extern crate reqwest;
extern crate select;
use rand::seq::SliceRandom;
use select::document::Document;
use select::predicate::{Name, Predicate};


pub async fn fn_of_the_day() -> FnInfo { // Result<FnInfo, &'static str> {
	let base_url_folder = String::from("http://www.ece.northwestern.edu/local-apps/matlabhelp/techdoc/ref/");
    let response = reqwest::get(format!("{}refbookl.html",base_url_folder))
    	.await.unwrap()
    	.text().await.unwrap();

    let html_document = Document::from_read(response.as_bytes()).unwrap();
	
	let fn_names_links: Vec<_> = html_document
        .find(Name("p").child(Name("a")))
        .filter(|el| !el.text().contains(" "))
        .map(|el| FnBasicInfo {name: el.text(),
        				  	   url: format!("{}{}",base_url_folder,el.attr("href").unwrap())})
        .collect();

	let mut rng = rand::thread_rng();
	let choice = fn_names_links.choose(&mut rng).unwrap().clone();
	let desc = fn_descriptor(choice.clone().url).await;

	let choice_with_desc = FnInfo {name: choice.name, url: choice.url, desc: desc};

	// choice
	choice_with_desc
}

async fn fn_descriptor(url: String) -> String {
	let response = reqwest::get(url)
    	.await.unwrap()
    	.text().await.unwrap();

    let html_document = Document::from_read(response.as_bytes()).unwrap();

    html_document
    	.find(Name("body").child(Name("p")))
    	.next()
    	.unwrap()
    	.text()
}

#[derive(Debug, Clone)]
struct FnBasicInfo {
	name: String,
	url: String
}

#[derive(Debug, Clone)]
pub struct FnInfo {
	pub name: String,
	pub url: String,
	pub desc: String
}
