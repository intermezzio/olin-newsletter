extern crate reqwest;
extern crate soup;
extern crate markup5ever_rcdom;
use rand::seq::SliceRandom;
use tokio::runtime::Handle;
use soup::Soup;
use soup::prelude::*;
use markup5ever_rcdom::Node;
use soup::NodeExt;
use std::error::Error;
use soup::prelude::*;

// #[derive(Debug, Clone)]
// struct FnInfo {
// 	name: String,
// 	desc: String,
// 	url: Option<String>,
// }

// fn get_fn_info(tr: Node) -> Result<FnInfo, &'static String> {
// 	let tds = tr.tag("td").find_all()
// 		.collect::<Vec<_>>();

// 	let name = tds[0].text();
// 	let desc = tds[1].text();
// 	let url = tds[0].tag("a").find().unwrap().get("href");

// 	Ok(FnInfo {name: name, desc: desc, url: url})
// }

pub async fn fn_of_the_day() -> Soup { // Result<FnInfo, &'static str> {
    let response = reqwest::get("https://www.mathworks.com/help/matlab/referencelist.html?type=function")
    	.await.unwrap()
    	.text().await.unwrap();

	let html_soup = Soup::new(&response);
	
	html_soup
	// let all_fn_info = html_soup
	// 	.tag("tr")
 //        .find_all()
 //        .map(get_fn_info)
		// .collect::<Vec<_>>();		

	// let mut rng = rand::thread_rng();
	// let choice = all_fn_info.choose(&mut rng);

	// choice
}