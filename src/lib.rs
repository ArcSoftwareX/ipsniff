use std::{
    io::{self, Write},
    net::{IpAddr, TcpStream},
    sync::mpsc::Sender,
};

pub fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, threads_count: u16, max_port: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        if TcpStream::connect((addr, port)).is_ok() {
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }

        if max_port - port <= threads_count {
            break;
        }

        port += threads_count;
    }
}
