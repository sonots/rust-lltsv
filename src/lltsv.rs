use std::collections::HashSet;
use std::collections::BTreeMap;
use std::io::{self, Read, BufRead};

pub struct Lltsv<'a> {
    keys: Vec<&'a str>,
    ignore_key_set: HashSet<&'a str>,
    no_key: bool,
    // func_append: tFuncAppend,
}

impl Lltsv<'_> {
    pub fn new<'a>(
        keys: Vec<&'a str>,
        ignore_keys: Vec<&'a str>,
        no_key: bool,
    ) -> Lltsv<'a> {
        let mut ignore_key_set = HashSet::new();
        for key in ignore_keys {
            ignore_key_set.insert(key);
        }

        Lltsv {
            keys,
            ignore_key_set,
            no_key,
        }
    }

    pub fn scan_and_write<R: Read>(&self, reader: R) -> io::Result<()> {
        let reader = io::BufReader::new(reader);

        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                continue;
            }
            let lvs = self.parse_ltsv(&line);
            let restructed = self.restruct_ltsv(lvs);
            println!("{}", restructed);
        }

        Ok(())
    }

    fn restruct_ltsv(&self, lvs: BTreeMap<&str, &str>) -> String {
        //  Sort in the order of -k, or follow the order of the input file.
        let keys: Vec<&str>; // just to make it alive orders_ref
        let orders_ref: &Vec<&str> = if self.keys.is_empty() {
            keys = lvs.keys().cloned().collect();
            &keys
        } else {
            &self.keys
        };
        if self.no_key {
            let mut selected = Vec::with_capacity(orders_ref.len());
            for label_ref in orders_ref {
                let value = lvs.get(label_ref).unwrap_or(&"");
                selected.push(*value);
            }
            selected.join("\t")
        } else {
            let mut selected = Vec::with_capacity(orders_ref.len());
            for label_ref in orders_ref {
                let label = label_ref.to_string();
                let value = lvs.get(label_ref).unwrap_or(&"");
                selected.push(label + ":" + value);
            }
            selected.iter().map(|s| s.as_str()).collect::<Vec<&str>>().join("\t")
        }
    }

    fn parse_ltsv<'a>(&'a self, line: &'a str) -> BTreeMap<&'a str, &'a str> {
        let columns: Vec<&str> = line.split('\t').collect();
        let mut lvs = BTreeMap::new();
        for column in columns {
            let lv: Vec<&str> = column.splitn(2, ':').collect();
            if lv.len() < 2 {
                continue;
            }
            let (label, value) = (lv[0], lv[1]);
            if self.ignore_key_set.contains(label) {
                continue;
            }
            lvs.insert(label, value);
        }
        lvs
    }
}