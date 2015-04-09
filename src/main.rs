extern crate getopts;
use getopts::Options;
use std::env;
use std::net::{UdpSocket,ToSocketAddrs};

pub fn send_to_anybar<A: ToSocketAddrs>(command: &str, addr: A) {
    // Until I figure out how to build a UdpSocket without binding,
    // this will have to do.
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.send_to(command.as_bytes(), addr).unwrap();
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("a", "addr", "address to connect to anybar. Default is 127.0.0.1:1738.", "PORT");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let addr = matches.opt_str("a").or(
                 Some("127.0.0.1:1738".to_string())
              ).unwrap();
    let command = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    send_to_anybar(&*command, &*addr);
}
