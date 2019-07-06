use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
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
    id: usize,
    name: String,
    // tags: HashSet<Tag>,
    writer: BufWriter<File>,
}

impl Metric {
    pub fn new(id: usize, name: &str, filename: &str) -> Metric {
        let file = File::create(filename).unwrap();
        Metric {
            id,
            name: String::from(name),
            // tags: HashSet<Tag>::new(),
            writer: BufWriter::new(file),
        }
    }
}

pub struct Database {
    name: String,
    metrics: Vec<Metric>,
    name_map: HashMap<String, usize>,
}

impl Database {
    pub fn new(name: &str) -> Database {
        fs::create_dir_all(name).unwrap();
        Database {
            name: String::from(name),
            metrics: Vec::new(),
            name_map: HashMap::new(),
        }
    }

    fn create_metric_filename(&self, name: &str) -> String {
        format!("{}/{}", self.name, name)
    }

    pub fn add_sample_u64(&mut self, id: usize, sample: Sample<u64, File>) {
        let writer: &mut BufWriter<File> = &mut self.metrics.get_mut(id).unwrap().writer;
        sample.write(writer);
    }

    pub fn metric_id(&self, name: &str) -> &usize {
        self.name_map.get(name).unwrap()
    }

    pub fn add_metric(&mut self, name: &str) -> usize {
        let id = self.metrics.len();
        let filename = self.create_metric_filename(name);
        let metric = Metric::new(id, name, &filename);
        self.metrics.push(metric);
        self.name_map.insert(String::from(name), id);
        id
    }
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
