use std::fs::File;

extern crate clap;
use clap::{App, Arg};

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
        keys = k.split(',').map(|s| s.to_string()).collect();
    }

    let no_key = matches.is_present("no-key");

    let mut ignore_keys = Vec::new();
    if keys.is_empty() {
        if let Some(ik) = matches.value_of("ignore-key") {
            ignore_keys = ik.split(',').map(|s| s.to_string()).collect();
        }
    }

    println!("keys: {:?}", keys);
    println!("no_key: {:?}", no_key);
    println!("ignore_keys: {:?}", ignore_keys);

    if let Some(filenames) = matches.values_of("FILENAME") {
        for filename in filenames {
            println!("filename: {}", filename);
            match File::open(filename) {
                Ok(mut _file) => {
                    // ファイルを読み込む処理をここに書く
                    // ファイルはスコープを抜けると自動的に閉じられます
                }
                Err(_e) => {
                    eprintln!("failed to open and read `{}`.", filename);
                    std::process::exit(1);
                }
            }
        }
    } else {
        let _stdin = std::io::stdin();
        // 標準入力から読み込む処理をここに書く
        // stdinはスコープを抜けると自動的に閉じられます
    }
}