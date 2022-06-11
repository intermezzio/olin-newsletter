extern crate reqwest;
use rand::seq::SliceRandom;
use tokio::runtime::Handle;
extern crate select;

use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

#[derive(Debug, Clone)]
struct FnInfo {
	name: String,
	url: String
}

pub async fn fn_of_the_day() -> () { // Result<FnInfo, &'static str> {
	let base_url_folder = String::from("http://www.ece.northwestern.edu/local-apps/matlabhelp/techdoc/ref/");
    let response = reqwest::get(format!("{}refbookl.html",base_url_folder))
    	.await.unwrap()
    	.text().await.unwrap();

    let html_document = Document::from_read(response.as_bytes()).unwrap();
	
	let fn_names_links: Vec<_> = html_document
        .find(Name("p").child(Name("a")))
        .filter(|el| !el.text().contains(" "))
        .map(|el| FnInfo {name: el.text(), url: format!("{}{}",base_url_folder,el.attr("href").unwrap())})
        .collect();

	let mut rng = rand::thread_rng();
	let choice = fn_names_links.choose(&mut rng).unwrap();

	println!("{:?}", choice);
	// choice
	choice;
}