#![feature(mpsc_select, drain)]
#[macro_use]


extern crate log;

mod simplelogger;
mod metric;
mod metric_store;

use metric::Metric;
use metric_store::{MetricStore};
use simplelogger::SimpleLogger;

use std::sync::mpsc::Sender;
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead};
use std::thread::spawn;

fn main() {
    let _ = SimpleLogger::init();
    let store = MetricStore::new();

    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).unwrap();
    info!("{:?}", addr);

    for stream in listener.incoming() {
        let tx = store.sender.clone();
        match stream {
            Ok(stream) => {
                spawn(move|| { 
                    handle_client(stream, tx);
                });
                ()
            },
            Err(e) => {
                println!("acceptor error: {:?}", e);
                ()
            }
        }
    }
}

fn handle_client(stream: TcpStream, sender: Sender<Metric>) {
    println!("new stream {}", stream.peer_addr().unwrap());

    let mut bufreader = BufReader::new(stream);

    loop {
        let buffer_string = &mut String::new();
        match bufreader.read_line(buffer_string) {
            Err(e) => panic!("Got an error {}", e),
            Ok(0) => return,
            Ok(_) => {
                if buffer_string.trim().len() > 0 {
                    let metric = Metric::new(buffer_string.to_owned());
                    match metric {
                        Ok(m) => {
                            sender.send(m).unwrap_or_else(|e| error!("send failed {:?}", e));
                            ()
                        },
                        Err(e) => error!("{:?}", e)
                    }                    
                }
                ()
            }
        }
    }
}



//<metric name>:<value>|g
