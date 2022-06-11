// use futures::join;
mod get_matlab;
mod get_chuck;

#[tokio::main]
async fn main() -> () {
    println!("Hello, world!");

    // create tasks for all api requests
    let matlab_fn = get_matlab::fn_of_the_day().await;
    let chuck_norris_quote = get_chuck::chuck_norris_quote().await;

    // todo: make all the api requests run on different threads here
    // join!(matlab_fn, chuck_norris_quote);

    println!("{:?}", matlab_fn);
    println!("{:?}", chuck_norris_quote);    
}
