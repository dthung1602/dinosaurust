use std::cmp::Ord;
use std::net::IpAddr;
use clap::{Parser, arg, value_parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(long, value_name = "IP", default_value = "0.0.0.0")]
    pub ip: IpAddr,

    #[arg(long, value_name = "PORT", default_value = "2053",
          value_parser = value_parser!(u32).range(1..65536))]
    pub port: u32,
}


pub fn load_config() -> Config {
    Config::parse()
}
