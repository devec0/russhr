mod logo;
mod args;
mod scanner;

fn main() {
    logo::display_logo();
    let parsed_config = args::parse().unwrap();
    scanner::start(parsed_config);
}
