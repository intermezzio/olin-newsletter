use futures::executor::block_on;
mod get_matlab;
mod get_chuck;

#[tokio::main]
async fn main() -> () {
    println!("Hello, world!");

    // let matlab_fn = get_matlab::fn_of_the_day().await;
    // println!("{:?}", matlab_fn);

    let chuck_norris_quote = get_chuck::chuck_norris_quote().await;
    println!("{:?}", chuck_norris_quote);    
}
