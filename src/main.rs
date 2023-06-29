use std::env;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Read, Write};
use std::path::Path;
use std::process::ExitCode;

struct Key {
    key: Vec<u8>,
    keylen: usize,
    index: usize,
}

impl Iterator for Key {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.key[self.index];
        if self.index == self.keylen {
            self.index = 0;
        } else {
            self.index += 1;
        }

        Some(value)
    }
}

impl Key {
    fn new(data: &str) -> Self {
        Self {
            key: data.as_bytes().to_owned(),
            keylen: data.len() - 1,
            index: 0,
        }
    }
}

const BUFSIZE: usize = 8192; //8 KiB

fn main() -> ExitCode {
    let mut buffer = vec![0; BUFSIZE];

    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    if argc < 3 {
        eprintln!("usage: xorcrypt <inputfile> <outputfile>");
        return ExitCode::FAILURE;
    }

    let mut input = String::new();
    print!("Key: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();

    let mut key = Key::new(input.trim());
    let mut file = BufReader::new(match File::open(&argv[1]) {
        Ok(fd) => fd,
        Err(_) => {
            eprintln!(
                "{}: Unable to open file (not found / no permission)",
                &argv[1]
            );
            return ExitCode::FAILURE;
        }
    });

    if Path::new(&argv[2]).exists() {
        eprintln!("{}: File already exists.", &argv[2]);
        return ExitCode::FAILURE;
    }

    let mut outfile = match File::create(&argv[2]) {
        Ok(fd) => fd,
        Err(_) => {
            eprintln!("{}: Unable to create file", &argv[2]);
            return ExitCode::FAILURE;
        }
    };

    let mut bytes_read;
    loop {
        bytes_read = file.read(&mut buffer).unwrap();

        if bytes_read == 0 {
            break;
        }

        for i in 0..bytes_read {
            buffer[i] ^= key.next().unwrap();
        }

        outfile.write_all(&buffer[..bytes_read]).unwrap();
    }

    ExitCode::SUCCESS
}
