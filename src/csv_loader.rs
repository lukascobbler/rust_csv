use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::marker::PhantomData;
use crate::csvable;

pub struct CsvLoader<'a, V>
{
    path: &'a str,
    phantom: PhantomData<V>
}

impl<V> CsvLoader<'_, V>
where
    V: csvable::ToCsv
{
    pub fn save(&self, objects: &HashMap<String, V>) {
        let mut f = OpenOptions::new().write(true).create(true).open(&self.path).unwrap();
        for (_, value) in objects {
            f.write_all(format!("{}\n", value.to_csv()).as_bytes()).expect("unable to write data");
        }
    }
    
    pub fn load(&mut self) -> HashMap<String, V> {
        let file = File::open(&self.path).expect("can't open file");
        let reader = BufReader::new(file);
        let mut temp_objects = HashMap::new();

        for line in reader.lines() {
            if let Ok(line) = line {
                let temp_obj = match V::from_csv(&line) {
                    Some(obj) => obj,
                    None => {
                        eprintln!("the object \"{}\" is missing some fields", line);
                        continue
                    }
                };
                temp_objects.insert(temp_obj.main_key(), temp_obj);
            }
        }

        temp_objects
    }

    pub fn new(path: &str) -> CsvLoader<V> {
        CsvLoader { path, phantom: PhantomData }
    }
}