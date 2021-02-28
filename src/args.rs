extern crate clap;

use clap::{App, Arg, ArgGroup, crate_version};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::string::String;

pub struct Config {
    rate: u32,
    users: Vec<String>,
    passwords: Vec<String>,
    hosts: Vec<String>
}

fn load_file(path:String) -> Result<Vec<String>, &'static str> {
    
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    
    let data: Vec<String> = buf.lines().collect::<Result<_, _>>().unwrap();

    return Ok(data);
}
     
pub fn parse() -> Result<Config, &'static str> {
    //Set up our app
    let config = App::new("russhr")
	.version(crate_version!())
	.about("Fast SSH login err... tester?")
	.args(&[
	    Arg::with_name("rate")
		.help("Maximum number of login attempts per second")
		.short("r")
		.long("rate")
		.default_value("100"),
	    Arg::with_name("user")
		.help("Username to test")
		.short("u")
		.long("user"),
	    Arg::with_name("pass")
		.help("Password to test")
		.short("p")
		.long("pass"),
	    Arg::with_name("host")
		.help("Host to test")
		.short("h")
		.long("host"),
	    Arg::with_name("userfile")
		.help("Path to a file contain a list of users")
		.short("U")
		.long("userfile"),
	    Arg::with_name("passfile")
		.help("Path to a file contain a list of passwords")
		.short("P")
		.long("passfile"),
	    Arg::with_name("hostfile")
		.help("Path to a file contain a list of hosts")
		.short("H")
		.long("hostfile"),
	])
	.group(ArgGroup::with_name("usergroup")
		   .args(&["user","userfile"])
		   .required(true))
	.group(ArgGroup::with_name("hostgroup")
		   .args(&["host","hostfile"])
		   .required(true))
	.group(ArgGroup::with_name("passgroup")
		   .args(&["pass","passfile"])
	       .required(true))
	.get_matches();

    let mut parsed_config = Config{
	rate: config.value_of("rate").unwrap().parse().unwrap(),
	users: Vec::new(),
	passwords: Vec::new(),
	hosts: Vec::new()
    };

    if config.is_present("user") {
	parsed_config.users.push(config.value_of("user").unwrap().parse().unwrap());
	
    }
    
    if config.is_present("userfile") {
	parsed_config.users.append(&mut load_file(config.value_of("userfile").unwrap().parse().unwrap()).unwrap());
    }

    if config.is_present("pass") {
	parsed_config.passwords.push(config.value_of("pass").unwrap().parse().unwrap());
    }
    
    if config.is_present("passfile") {
	parsed_config.passwords.append(&mut load_file(config.value_of("passfile").unwrap().parse().unwrap()).unwrap());
    }

    if config.is_present("host") {
	parsed_config.hosts.push(config.value_of("host").unwrap().parse().unwrap());
    }

    if config.is_present("hostfile") {
	parsed_config.hosts.append(&mut load_file(config.value_of("hostfile").unwrap().parse().unwrap()).unwrap());
    }
    //validate users
    //validate hosts
    //validate passwords
    //set rate
    return Ok(parsed_config)
}
