use std::fs::File;

use clap::Parser;

mod lltsv;
use lltsv::Lltsv;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// lltsv - List specified keys of LTSV (Labeled Tab Separated Values)
struct Opts {
    #[clap(short = 'k', long, help = "keys to output (multiple keys separated by ,)")]
    key: Option<String>,

    #[clap(short = 'K', long, help = "output without keys (and without color)")]
    no_key: bool,

    #[clap(short = 'i', long, help = "ignored keys to output (multiple keys separated by ,)")]
    ignore_key: Option<String>,

    #[clap(help = "Set the input file(s)")]
    filename: Option<Vec<String>>,
}

fn main() {
    let opts = Opts::parse();

    let mut keys = Vec::new();
    if let Some(k) = opts.key.as_deref() {
        keys = k.split(',').collect();
    }

    let no_key = opts.no_key;

    let mut ignore_keys = Vec::new();
    if keys.is_empty() {
        if let Some(ik) = opts.ignore_key.as_deref() {
            ignore_keys = ik.split(',').collect();
        }
    }

    let lltsv = Lltsv::new(keys, ignore_keys, no_key);

    if let Some(filenames) = &opts.filename {
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