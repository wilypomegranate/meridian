// use serde::{Debug, Deserialize};
use crossbeam::crossbeam_channel::unbounded;
use crossbeam::crossbeam_channel::Receiver;
use meridian::*;
use serde::Deserialize;
use serde_json::Result;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::channel;
use std::thread::sleep;
use std::time::Duration;
use threadpool::ThreadPool;

// fn main() -> Result<(), Box<dyn Error>> {
fn main() {
    // let stdin = io::stdin();

    let pool = ThreadPool::new(4);

    let listener = TcpListener::bind("127.0.0.1:2003").unwrap();
    let (tx, rx) = unbounded();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!(
            "Connection established on {}:{}.",
            stream.peer_addr().unwrap().ip(),
            stream.peer_addr().unwrap().port(),
        );

        let rx = rx.clone();

        pool.execute(|| {
            handle_stream(stream, rx);
        });
    }

    // let (tx, rx) = unbounded();
    // let rx = rx.clone();
    // pool.execute(|| {
    // if let Ok(res) = rx.recv() {
    //     println!("{}", res);
    //     sleep(Duration::from_secs(1));
    // }
    // });

    // assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8);
    loop {
        // tx.send(1);
        // sleep(Duration::from_millis(10));
    }
}

fn handle_stream(stream: TcpStream, rx: Receiver<Sample<f64, File>>) {
    // let mut buffer = [0; 2048];
    // stream.read_line(&mut buffer).unwrap();
    let buffered_reader = io::BufReader::new(stream);
    let mut buffered_writer = io::BufWriter::new(File::create("test.out").unwrap());
    for line in buffered_reader.lines() {
        // This can fail if data isn't UTF-8.
        if let Ok(line) = line {
            let msg: NetdataMessage =
                serde_json::from_str(&line).expect("Got invalid JSON data from netdata.");
            // let ts: u64 = msg.timestamp as u64 * 1_000_000_000;
            // let sample: Sample<f64, File> = Sample::new(ts, msg.value);
            // sample.write(&mut buffered_writer);
            println!("{:#?}", msg);
        }
    }
    // println!("{}", String::from_utf8_lossy(&buffer));
}

// {"prefix":"netdata","hostname":"ouranos","chart_id":"system.interrupts","chart_name":"system.interrupts","chart_family":"interrupts","chart_context": "system.interrupts","chart_type":"system","units": "interrupts/s","id":"NMI","name":"NMI","value":0.7234295,"timestamp": 1562193888}
