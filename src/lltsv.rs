use std::collections::HashMap;
use std::io::{self, Read, BufRead};

pub struct Lltsv<'a> {
    keys: Vec<&'a str>,
    ignore_key_map: HashMap<&'a str, ()>,
    no_key: bool,
    // func_append: tFuncAppend,
}

impl Lltsv<'_> {
    pub fn new<'a>(
        keys: Vec<&'a str>,
        ignore_keys: Vec<&'a str>,
        no_key: bool,
    ) -> Lltsv<'a> {
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
            let (lvs, labels) = self.parse_ltsv(&line);
            let restructed = self.restruct_ltsv(lvs, labels);
            println!("{}", restructed);
        }

        Ok(())
    }

    fn restruct_ltsv(&self, lvs: HashMap<String, String>, labels: Vec<&str>) -> String {
        //  Sort in the order of -k, or follow the order of the input file.
        let orders = if self.keys.is_empty() {
            &labels
        } else {
            &self.keys
        };
        // Make vector with enough capacity so that push does not newly create object
        let mut selected = Vec::with_capacity(orders.len());
        let default_value = String::from("");
        for label_ref in orders {
            if self.ignore_key_map.contains_key(label_ref) {
                continue;
            }
            // TODO: any ways to avoid copying?
            let label = label_ref.to_string();
            let value = lvs.get(&label).unwrap_or(&default_value);
            if self.no_key {
                selected.push(value.to_string());
            } else {
                selected.push(label + ":" + value);
            }
        }
        selected.iter().map(|s| s.as_str()).collect::<Vec<&str>>().join("\t")
    }

    fn parse_ltsv<'a>(&'a self, line: &'a str) -> (HashMap<String, String>, Vec<&str>) {
        let columns: Vec<&str> = line.split('\t').collect();
        let mut lvs = HashMap::new();
        let mut labels = Vec::with_capacity(columns.len());
        for column in columns {
            let lv: Vec<&str> = column.splitn(2, ':').collect();
            if lv.len() < 2 {
                continue;
            }
            let (label, value) = (lv[0], lv[1]);
            labels.push(label);
            lvs.insert(label.to_string(), value.to_string());
        }
        (lvs, labels)
    }
}