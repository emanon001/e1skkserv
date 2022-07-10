use anyhow::{anyhow, Error, Result};
use encoding::all::EUC_JP;
use encoding::{DecoderTrap, EncoderTrap, Encoding};
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

pub fn serve(host: &str) -> Result<()> {
    let listener = TcpListener::bind(host)?;
    debug!("bind {}", host);
    loop {
        let (stream, _) = listener.accept()?;
        let context = ServerContext {
            host: host.to_string(),
        };
        thread::spawn(move || {
            handler(stream, context).unwrap_or_else(|e| error!("{:?}", e));
        });
    }
}

struct ServerContext {
    host: String,
}

fn handler(mut stream: TcpStream, context: ServerContext) -> Result<()> {
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
            Request::Convert(s) => convert(&s),
            Request::Version => skkserv_version(),
            Request::Host => skkserv_host(&context.host),
            Request::Complete(s) => complete(&s),
        };
        debug!("response: {}", res);
        let res = encode_response(&res)?;
        stream.write_all(&res)?;
    }
}

fn skkserv_version() -> String {
    format!(
        "{}.{}.{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
    )
}

fn skkserv_host(address: &str) -> String {
    format!("{}", address)
}

fn convert(s: &str) -> String {
    // TODO: 実装
    "4\n".to_string()
}

fn complete(_req: &str) -> String {
    // 未対応
    let res = "4\n".to_string();
    return res;
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
