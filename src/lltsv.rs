use std::collections::HashMap;
use std::io::{self, Read, BufRead};

pub struct Lltsv {
    keys: Vec<String>,
    ignore_key_map: HashMap<String, ()>,
    no_key: bool,
    // func_append: tFuncAppend,
}

impl Lltsv {
    pub fn new(
        keys: Vec<String>,
        ignore_keys: Vec<String>,
        no_key: bool,
    ) -> Lltsv {
        let mut ignore_key_map = HashMap::new();
        for key in ignore_keys {
            ignore_key_map.insert(key, ());
        }

        Lltsv {
            keys,
            ignore_key_map,
            no_key,
        }
    }

    pub fn scan_and_write<R: Read>(&self, reader: R) -> io::Result<()> {
        let reader = io::BufReader::new(reader);

        for line in reader.lines() {
            let line = line?;
            let (lvs, keys) = self.parse_ltsv(&line);
            let restructed = self.restruct_ltsv(lvs, keys);
            println!("{}", restructed);
        }

        Ok(())
    }

    fn restruct_ltsv(&self, lvs: HashMap<String, String>, labels: Vec<String>) -> String {
        //  Sort in the order of -k, or follow the order of the input file.
        let mut orders = self.keys.clone();
        if self.keys.is_empty() {
            orders = labels;
        }
        // Make vector with enough capacity so that push does not newly create object
        let mut selected = Vec::with_capacity(orders.len());
        let default_value = String::from("");
        for label in orders {
            if self.ignore_key_map.contains_key(&label) {
                continue;
            }
            let value = lvs.get(&label).unwrap_or(&default_value);
            if self.no_key {
                selected.push(value.to_string());
            } else {
                selected.push(format!("{}:{}", label, value.to_string()));
            }
        }
        selected.join("\t")
    }

    fn parse_ltsv(&self, line: &str) -> (HashMap<String, String>, Vec<String>) {
        let columns: Vec<&str> = line.split('\t').collect();
        let mut lvs = HashMap::new();
        let mut labels = Vec::with_capacity(columns.len());
        for column in columns {
            let lv: Vec<&str> = column.splitn(2, ':').collect();
            if lv.len() < 2 {
                continue;
            }
            let (label, value) = (lv[0].to_string(), lv[1].to_string());
            labels.push(label.clone());
            lvs.insert(label, value);
        }
        (lvs, labels)
    }
}