extern crate tokio;
mod logo;
mod args;
mod scanner;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    logo::display_logo();
    let parsed_config = args::Config::parse().unwrap();
    let scanner = scanner::Scanner::new(parsed_config).unwrap();
    scanner.run().await.unwrap();
    Ok(())
}
