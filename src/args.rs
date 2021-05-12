extern crate clap;

use clap::{App, Arg, ArgGroup, crate_version};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::string::String;

#[derive(Debug)]
pub struct Config {
    pub limit: u64,
    pub users: Vec<String>,
    pub passwords: Vec<String>,
    pub hosts: Vec<String>,
    pub login_count: usize,
    pub logins: Vec<Login>,
}

#[derive(Clone, Debug)]
pub struct Login {
    pub host: String,
    pub user: String,
    pub password: String,
}

impl Config {
    
    fn load_file(path:&str) -> Result<Vec<String>, &'static str> {
    
	let file = File::open(path).unwrap();
	let buf = BufReader::new(file);
	
	let data: Vec<String> = buf.lines().collect::<Result<_, _>>().unwrap();
	
	return Ok(data);
    }
     
    pub fn parse() -> Result<Self, &'static str> {
	//Set up our app
	let config = App::new("russhr")
	    .version(crate_version!())
	    .about("Fast SSH login err... tester?")
	    .args(&[
		Arg::with_name("limit")
		    .help("Limit concurrent login attempts")
		    .value_name("LIMIT")
		    .short("l")
		    .long("limit")
		    .default_value("10"),
		Arg::with_name("user")
		    .help("Username to test")
		    .value_name("USER")
		    .short("u")
		    .long("user"),
		Arg::with_name("pass")
		    .help("Password to test")
		    .value_name("PASS")
		    .short("p")
		    .long("pass"),
		Arg::with_name("host")
		    .help("Host to test")
		    .value_name("HOST")
		    .short("h")
		    .long("host"),
		Arg::with_name("userfile")
		    .help("Path to a file contain a list of users")
		    .value_name("USERFILE")
		    .short("U")
		    .long("userfile"),
		Arg::with_name("passfile")
		    .help("Path to a file contain a list of passwords")
		    .value_name("PASSFILE")
		    .short("P")
		    .long("passfile"),
		Arg::with_name("hostfile")
		    .help("Path to a file contain a list of hosts")
		    .value_name("HOSTFILE")
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
	    limit: config.value_of("limit").unwrap().parse().unwrap(),
	    users: Vec::new(),
	    passwords: Vec::new(),
	    hosts: Vec::new(),
	    login_count: 0,
	    logins: Vec::new(),
	};

	if let Some(user) = config.value_of("user") {
	    parsed_config.users.push(user.to_string());
	}
	
	if let Some(userfile) = config.value_of("userfile") {
	    parsed_config.users.append(&mut Config::load_file(userfile).unwrap());
	}
	
	if let Some(pass) = config.value_of("pass") {
	    parsed_config.passwords.push(pass.to_string());
	}
    
	if let Some(passfile) = config.value_of("passfile") {
	    parsed_config.passwords.append(&mut Config::load_file(passfile).unwrap());
	}

	if let Some(host) = config.value_of("host") {
	    parsed_config.hosts.push(host.to_string());
	}
    
	if let Some(hostfile) = config.value_of("hostfile") {
	    parsed_config.hosts.append(&mut Config::load_file(hostfile).unwrap());
	}

	for host in parsed_config.hosts.iter() {
	    for user in parsed_config.users.iter() {
		for pass in parsed_config.passwords.iter() {
		    let host = host.clone();
		    let user = user.clone();
		    let pass = pass.clone();
		    parsed_config.logins.push(Login{host: host, user: user, password: pass});
		    parsed_config.login_count = parsed_config.login_count + 1;
		}
            }
	}
	
	return Ok(parsed_config)
    }
}
