// use futures::join;
mod get_matlab;
mod get_chuck;
mod get_animal;
mod get_stock;
mod get_config;
mod get_news;
mod get_weather;

#[tokio::main]
async fn main() -> () {
    println!("Hello, world!");

    // get config variables
    let config_vars = get_config::load();

    // create tasks for all api requests
    let news_headlines = get_news::download(
        config_vars.newsapi.unwrap())
        .await;
    let matlab_fn = get_matlab::download().await;
    let chuck_norris_quote = get_chuck::download().await;
    let animal = get_animal::download().await;
    let forecast = get_weather::download(
        config_vars.weatherbit.unwrap()
        ).await;
    // this fails if the api is run from 9am - 4pm ET, will fix later
    // let gspc_data = get_stock::download("^GSPC").await;
    
    // todo: make all the api requests run on different threads here
    // join!(matlab_fn, chuck_norris_quote, animal);

    println!("{:?}", news_headlines);
    println!("{:?}", matlab_fn);
    println!("{:?}", chuck_norris_quote);
    println!("{:?}", animal);
    println!("{:?}", forecast);
    // println!("{:?}", gspc_data);
}
