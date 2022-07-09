use anyhow::{anyhow, Error, Result};
use log::{debug, error};
use regex::Regex;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;
use std::{str, thread};

#[derive(Debug)]
enum Request {
    Disconnect,
    Convert(String),
    Version,
    Completion(String),
}

impl FromStr for Request {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if s.len() == 0 {
            return Err(anyhow!("invalid request"));
        }

        // disconnect
        {
            if s == "0" {
                return Ok(Self::Disconnect);
            };
        }

        // convert
        {
            let re = Regex::new(r"\A1(.+) \z")?;
            if let Some(cap) = re.captures(s) {
                return Ok(Self::Convert(cap[1].to_string()));
            }
        }

        // version
        {
            if s == "2" {
                return Ok(Self::Version);
            };
        }

        // completion
        {
            let re = Regex::new(r"\A3(.+) \z")?;
            if let Some(cap) = re.captures(s) {
                return Ok(Self::Completion(cap[1].to_string()));
            }
        }

        Err(anyhow!("invalid request"))
    }
}

pub fn serve(address: &str) -> Result<()> {
    let listener = TcpListener::bind(address)?;
    debug!("bind {}", address);
    loop {
        let (stream, _) = listener.accept()?;
        thread::spawn(move || {
            handler(stream).unwrap_or_else(|e| error!("{:?}", e));
        });
    }
}

fn handler(mut stream: TcpStream) -> Result<()> {
    debug!("Handling data from {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024];
    loop {
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            debug!("Connection closed.");
            return Ok(());
        }
        let s = str::from_utf8(&buffer[..nbytes])?;
        debug!("request: '{}'", s);
        let req = Request::from_str(s)?;
        debug!("parsed request: {:?}", req);
        // TODO
        match req {
            Request::Disconnect => {}
            Request::Convert(s) => {}
            Request::Version => {}
            Request::Completion(s) => {}
        }
        // stream.write_all("".as_bytes())?;
    }
}
