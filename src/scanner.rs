use crate::args;
use std::string::String;
use std::vec::Vec;
use tokio::runtime::Runtime;

pub fn scan_host(host:String, users:Vec<String>, passwords:Vec<String>) {
}

pub fn start(config:args::Config) {
    // set up runner
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
	eprintln!("Hi");
    })
    // create queue
    
    // rate limit requests
    // XXX: PID for auto tuning
}
