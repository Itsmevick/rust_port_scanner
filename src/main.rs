use std::env;
use std::net::IpAddr;
use std::str::FromStr;
use std::process;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::net::TcpStream;
use std::io::{self, Write};
use std::time::Instant;

#[derive(Debug)]
struct Arguments {
    flag: String,
    ipaddress: IpAddr,
    threads_no: u16,
}

const MAX_PORT: u16 = 65535;

impl Arguments {
    fn new(arguments: &[String]) -> Result<Arguments, &'static str> {
        if arguments.len() < 2 {
            return Err("Too few arguments");
        } else if arguments.len() > 4 {
            return Err("Too many arguments");
        }
        let flag = arguments[1].clone();
        if flag.contains("-h") || flag.contains("-help") && arguments.len() == 2 {
            println!("Usage: <flag> <ip address> <number of threads>");
            return Err("help");
        } else if flag.contains("-h") || flag.contains("-help") {
            return Err("failed! wrong arguments");
        } else if flag.contains("-j") {
            let ipaddress = match IpAddr::from_str(&arguments[2]) {
                Ok(s) => s,
                Err(_) => return Err("unrecognizable ip address"),
            };

            let threads_no = match arguments[3].parse::<u16>() {
                Ok(s) => s,
                Err(_) => return Err("wrong format for the number of threads"),
            };

            return Ok(Arguments {
                flag,
                ipaddress,
                threads_no,
            });
        } else {
            Err("invalid syntax")
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, address: IpAddr, threads: u16){
    let mut port = start_port + 1;
    loop {
        match TcpStream::connect((address, port)){
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            },
            Err(_) => {}
        }
        if (MAX_PORT - port) < threads{
            break;
        }
        port += threads;
    }
}


fn main() {
    let instant: Instant = Instant::now();
    let args: Vec<String> = env::args().collect();
    let cli_args = match Arguments::new(&args){
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            process::exit(0);
        },
    };
    let _flag = cli_args.flag;
    let ip = cli_args.ipaddress;
    let threads = cli_args.threads_no;

    let (tx, rx) = channel();
    
    for i in 0..threads{
        let tx1 = tx.clone();
        thread::spawn(move ||{
            scan(tx1,i, ip,threads);
        });
    }

    let mut output = vec![];
    drop(tx);
    for i in rx{
        output.push(i);
    }
    println!("");
    output.sort();
    for i in output{
        println!("{} is open", i);
    }
    println!("");
    println!("Time taken to retrieve open ports: {:?}", instant.elapsed());
}
