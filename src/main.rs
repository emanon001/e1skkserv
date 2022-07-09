use log::error;
use std::env;
use structopt::StructOpt;

use e1skkserv::server;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long, default_value = "1178", help = "[port]")]
    port: u32,
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let opt = Opt::from_args();
    let address = format!("localhost:{}", opt.port);
    server::serve(&address).unwrap_or_else(|e| error!("{}", e));
}
