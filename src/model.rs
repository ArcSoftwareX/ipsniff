// pub struct Config {
//     pub ipaddr: IpAddr,
//     pub threads: u16,
// }

// impl Config {
//     pub fn new(args: &[String]) -> Result<Config, &str> {
//         if args.is_empty() {
//             return Err("Not enough arguments");
//         }
//         if args.len() > 3 {
//             return Err("Too many arguments");
//         }

//         let ipaddr = &args[0];
//         if let Ok(ipaddr) = IpAddr::from_str(ipaddr) {
//             return Ok(Config { ipaddr, threads: 4 });
//         }

//         let flag = &args[0];
//         if flag.contains("-h") || flag.contains("--help") && args.len() == 1 {
//             println!("Usage: ipsniff [IP_ADDR]\n-t to select threads count");
//             process::exit(0)
//         }

//         if flag.contains("-t") {
//             let ipaddr = match IpAddr::from_str(&args[2]) {
//                 Ok(s) => s,
//                 Err(_) => return Err("Failed to parse IP address"),
//             };
//             let threads = match args[1].parse::<u16>() {
//                 Ok(t) => t,
//                 Err(_) => return Err("Failed to parse threads count"),
//             };

//             return Ok(Config { ipaddr, threads });
//         }

//         Err("Invalid syntax")
//     }
// }

use clap::Parser;
use std::net::IpAddr;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    // Number of threads to use
    #[arg(short, long)]
    pub threads: u16,

    // IP address to scan
    #[arg(short, long)]
    pub ipaddr: IpAddr,

    #[arg(short, long)]
    pub maxport: u16,
}
