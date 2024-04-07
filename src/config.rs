use std::net::IpAddr;

use clap::{arg, value_parser, Parser};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(long, value_name = "IP", default_value = "0.0.0.0")]
    pub ip: IpAddr,

    #[arg(long, value_name = "PORT", default_value = "2053",
          value_parser = value_parser!(u32).range(1..65536))]
    pub port: u32,

    #[arg(long, value_name = "IP", default_value = "8.8.8.8")]
    pub forward_server_ip: IpAddr,

    #[arg(long, value_name = "PORT", default_value = "53",
          value_parser = value_parser!(u32).range(1..65536))]
    pub forward_server_port: u32,
}

impl Config {
    pub fn socket_address_str(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

pub fn load_config() -> Config {
    Config::parse()
}
