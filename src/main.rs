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
    let config_vars = get_config::api_keys_and_emails();

    // create tasks for all api requests
    let news_headlines = get_news::top_headlines(
        config_vars.newsapi.unwrap())
        .await;
    let matlab_fn = get_matlab::fn_of_the_day().await;
    let chuck_norris_quote = get_chuck::chuck_norris_quote().await;
    let animal = get_animal::rand_animal().await;
    let forecast = get_weather::hourly_forecast(
        config_vars.weatherbit.unwrap()
        ).await;
    // this fails if the api is run from 9am - 4pm ET, will fix later
    // let gspc_data = get_stock::ohlc("^GSPC").await;
    
    // todo: make all the api requests run on different threads here
    // join!(matlab_fn, chuck_norris_quote, animal);

    println!("{:?}", news_headlines);
    println!("{:?}", matlab_fn);
    println!("{:?}", chuck_norris_quote);
    println!("{:?}", animal);
    println!("{:?}", forecast);
    // println!("{:?}", gspc_data);
}
