use futures::executor::block_on;
mod get_matlab;

#[tokio::main]
async fn main() -> () {
    println!("Hello, world!");
    let matlab_fn = get_matlab::fn_of_the_day().await;
    println!("{:?}", matlab_fn.text());
}
