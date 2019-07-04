use byteorder::{NativeEndian, WriteBytesExt};
use csv::Reader;
use meridian::*;
use std::fs::File;
use std::io;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    let mut csv_reader = Reader::from_reader(io::stdin());
    let mut count: usize = 0;

    let file = File::create("test.out")?;
    let mut buffered = BufWriter::new(file);

    for result in csv_reader.records() {
        for item in result.iter() {
            let mut iter = item.iter();
            let ts: u64 = iter.next().unwrap().parse().unwrap();
            let value: u64 = iter.next().unwrap().parse().unwrap();
            let sample: Sample<u64, File> = Sample::new(ts, value);
            sample.write(&mut buffered);
            // buffered.write_u64::<NativeEndian>(ts).unwrap();
            // buffered.write_u64::<NativeEndian>(value).unwrap();
            // println!("ts: {} value: {}", ts, value);
            count += 1;
        }
    }
    println!("Parsed {} entries.", count);
    // let handle = stdin.lock();
    // for line in handle.lines() {
    //     println!("{}", line.unwrap());
    // }

    Ok(())
}
