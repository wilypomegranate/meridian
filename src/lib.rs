use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use std::collections::HashSet;
use std::io::{BufWriter, Write};

#[derive(serde::Deserialize, Debug)]
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
    pub value: f64,
    pub timestamp: u32,
}

struct Tag {
    name: String,
    value: String,
}

struct Metric {
    name: String,
    tags: HashSet<Tag>,
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
        writeable.write_u64::<LittleEndian>(self.ts).unwrap();
    }
}

impl<U> Sample<i32, U>
where
    U: Write,
{
    pub fn write(&self, writeable: &mut BufWriter<U>) {
        {
            self.write_ts(writeable);
        }
        writeable.write_i32::<LittleEndian>(self.value).unwrap();
    }
}

impl<U> Sample<f64, U>
where
    U: Write,
{
    pub fn write(&self, writeable: &mut BufWriter<U>) {
        {
            self.write_ts(writeable);
        }
        writeable.write_f64::<LittleEndian>(self.value).unwrap();
    }
}

impl<U> Sample<u64, U>
where
    U: Write,
{
    pub fn write(&self, writeable: &mut BufWriter<U>) {
        {
            self.write_ts(writeable);
        }
        writeable.write_u64::<LittleEndian>(self.value).unwrap();
    }
}

struct MetricWriter {
    metrics: Vec<Metric>,
}
