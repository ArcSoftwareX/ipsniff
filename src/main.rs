use std::{sync::mpsc, thread};

use clap::Parser;
use ipsniff::scan;
use model::Args;

use indicatif::{ProgressBar, ProgressStyle};

mod model;

fn main() {
    let args = Args::parse();

    let (tx, rx) = mpsc::channel();

    let bar = ProgressBar::new(args.maxport as u64);
    bar.set_style(ProgressStyle::default_bar().progress_chars("#>-"));

    for i in 0..args.threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, args.ipaddr, args.threads, args.maxport);
        });
    }
    drop(tx);

    let mut ports = Vec::new();

    for port in rx {
        if port as u64 > bar.position() {
            bar.set_position(port as u64);
        }
        ports.push(port);
    }

    bar.finish_with_message("Finished scanning");

    ports.sort();

    if ports.is_empty() {
        println!("No open ports found");
    }

    for port in ports {
        println!("Port {port} is open");
    }
}

// How it should work:

// ipsniff -h
// ipsniff -t 100 192.168.1.1
// ipsniff 192.168.1.1
