use crate::command;
use anyhow::{anyhow, Error, Result};
use encoding::all::EUC_JP;
use encoding::{DecoderTrap, EncoderTrap, Encoding};
use log::{debug, error};
use regex::Regex;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;
use std::{str, thread};

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
}

pub fn serve(config: ServerConfig) -> Result<()> {
    let host = &config.host;
    let listener = TcpListener::bind(&host)?;
    debug!("bind {}", host);
    loop {
        let (stream, _) = listener.accept()?;
        let config = config.clone();
        thread::spawn(move || {
            handler(stream, config).unwrap_or_else(|e| error!("{:?}", e));
        });
    }
}

#[derive(Debug)]
enum Request {
    Disconnect,
    Convert(String),
    Version,
    Host,
    Complete(String),
}

impl FromStr for Request {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if s.len() == 0 {
            return Err(anyhow!("invalid request"));
        }

        // 0: disconnect
        {
            if s == "0" {
                return Ok(Self::Disconnect);
            };
        }

        // 1: convert
        {
            let re = Regex::new(r"\A1(.+) \z")?;
            if let Some(cap) = re.captures(s) {
                return Ok(Self::Convert(cap[1].to_string()));
            }
        }

        // 2: version
        {
            if s == "2" {
                return Ok(Self::Version);
            };
        }

        // 3: host
        {
            if s == "3" {
                return Ok(Self::Host);
            };
        }

        // 4: complete
        {
            let re = Regex::new(r"\A4(.+) \z")?;
            if let Some(cap) = re.captures(s) {
                return Ok(Self::Complete(cap[1].to_string()));
            }
        }

        Err(anyhow!("invalid request"))
    }
}

fn handler(mut stream: TcpStream, config: ServerConfig) -> Result<()> {
    debug!("Handling data from {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024];
    loop {
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            debug!("Connection closed.");
            return Ok(());
        }

        // コマンドを実行
        let s = decode_request(&buffer[..nbytes])?;
        debug!("request: '{}'", s);
        let req = Request::from_str(&s)?;
        debug!("parsed request: {:?}", req);
        let res = match req {
            Request::Disconnect => {
                debug!("Connection closed.");
                return Ok(());
            }
            Request::Convert(s) => command::convert(&s),
            Request::Version => command::skkserv_version(),
            Request::Host => command::skkserv_host(&config.host),
            Request::Complete(s) => command::complete(&s),
        };
        debug!("response: {}", res);
        let res = encode_response(&res)?;
        stream.write_all(&res)?;
    }
}

fn decode_request(req: &[u8]) -> Result<String> {
    if let Ok(s) = EUC_JP.decode(req, DecoderTrap::Strict) {
        Ok(s)
    } else {
        Err(anyhow!("dedcode failed"))
    }
}

fn encode_response(res: &str) -> Result<Vec<u8>> {
    if let Ok(s) = EUC_JP.encode(&res, EncoderTrap::Strict) {
        Ok(s)
    } else {
        Err(anyhow!("encode failed"))
    }
}
