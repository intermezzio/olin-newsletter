// use futures::join;
mod get_matlab;
mod get_chuck;
mod get_animal;

#[tokio::main]
async fn main() -> () {
    println!("Hello, world!");

    // create tasks for all api requests
    let matlab_fn = get_matlab::fn_of_the_day().await;
    let chuck_norris_quote = get_chuck::chuck_norris_quote().await;
    let animal = get_animal::rand_animal().await;

    // todo: make all the api requests run on different threads here
    // join!(matlab_fn, chuck_norris_quote, animal);

    println!("{:?}", matlab_fn);
    println!("{:?}", chuck_norris_quote);
    println!("{:?}", animal);
}
