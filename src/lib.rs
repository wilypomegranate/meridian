use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use std::io::{BufWriter, Write};

#[derive(serde::Deserialize)]
pub struct NetdataMessage {
    prefix: String,
    hostname: String,
    chart_id: String,
    chart_name: String,
    chart_family: String,
    chart_type: String,
    units: String,
    id: String,
    name: String,
    value: f64,
    timestamp: u32,
}

struct Tag {
    name: String,
    value: String,
}

struct Metric {
    name: String,
    tags: Vec<Tag>,
}

pub struct Sample<T, U>
where
    U: Write,
{
    ts: u64,
    value: T,
    marker: ::std::marker::PhantomData<U>,
}

impl<T, U> Sample<T, U>
where
    U: Write,
{
    pub fn new(ts: u64, value: T) -> Sample<T, U> {
        Sample {
            ts,
            value,
            marker: std::marker::PhantomData,
        }
    }
    fn write_ts(&self, writeable: &mut BufWriter<U>) {
        writeable.write_u64::<LittleEndian>(self.ts);
    }
}

impl<U> Sample<i32, U>
where
    U: Write,
{
    // fn new(ts: u64, value: i32) -> Sample<i32> {
    //     Sample<i32>{ts, value}
    // }
    pub fn write(&self, writeable: &mut BufWriter<U>) {
        {
            self.write_ts(writeable);
        }
        writeable.write_i32::<LittleEndian>(self.value);
    }
}

struct MetricWriter {
    metrics: Vec<Metric>,
}
