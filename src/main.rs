extern crate tokio;
mod logo;
mod args;
mod scanner;

#[tokio::main]
async fn main() {
    logo::display_logo();
    let parsed_config = args::parse().unwrap();
    let _result = scanner::start(&parsed_config).await;
}
