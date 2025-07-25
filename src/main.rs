use std::fs::File;
use std::io::{Read, Write, stdin, stdout};
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    decode: bool,

    #[clap(default_value = "-")]
    path: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let mut buffer = vec![];

    if cli.path == PathBuf::from("-") {
        stdin()
            .lock()
            .read_to_end(&mut buffer)
            .expect("Failed to read stdin");
    } else {
        File::open(cli.path)
            .expect("Failed to open file")
            .read_to_end(&mut buffer)
            .expect("Failed to read file");
    };

    if cli.decode {
        let buffer = String::from_utf8(buffer).expect("Failed to read as UTF-8 string");

        let decoded =
            base32768::decode(buffer.trim()).expect("Failed to decode input");

        stdout()
            .lock()
            .write_all(&decoded)
            .expect("Failed to write to stdout");
    } else {
        println!("{}", base32768::encode(&buffer));
    }
}
