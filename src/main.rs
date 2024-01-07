use std::fs::File;

extern crate clap;
use clap::{App, Arg};

mod lltsv;
use lltsv::Lltsv;

fn main() {
    let app = App::new("lltsv")
        .author("Naotoshi Seo <sonots@gmail.com>")
        .arg(
            Arg::with_name("key")
                .short('k')
                .help("keys to output (multiple keys separated by ,)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("no-key")
                .short('K')
                .help("output without keys (and without color)")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("ignore-key")
                .short('i')
                .help("ignored keys to output (multiple keys separated by ,)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("FILENAME")
                .help("Set the input file(s)")
                .multiple(true)
                .takes_value(true)
        );
    let matches = app.get_matches();

    let mut keys = Vec::new();
    if let Some(k) = matches.value_of("key") {
        keys = k.split(',').collect();
    }

    let no_key = matches.is_present("no-key");

    let mut ignore_keys = Vec::new();
    if keys.is_empty() {
        if let Some(ik) = matches.value_of("ignore-key") {
            ignore_keys = ik.split(',').collect();
        }
    }

    let lltsv = Lltsv::new(keys, ignore_keys, no_key);

    if let Some(filenames) = matches.values_of("FILENAME") {
        for filename in filenames {
            match File::open(filename) {
                Ok(file) => {
                    match lltsv.scan_and_write(file) {
                        Ok(_) => {}
                        Err(_e) => {
                            eprintln!("failed to process `{}`.", filename);
                            std::process::exit(1);
                        }
                    }
                }
                Err(_e) => {
                    eprintln!("failed to open and read `{}`.", filename);
                    std::process::exit(1);
                }
            }
        }
    } else {
        let stdin = std::io::stdin();
        match lltsv.scan_and_write(stdin.lock()) {
            Ok(_) => {}
            Err(_e) => {
                eprintln!("failed to process stdin.");
                std::process::exit(1);
            }
        }
    }
}