extern crate serde_bencode;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_bytes;

use serde_bencode::de;

use std::fs::{File, read_dir};
use std::io::{self, Read};
use std::io::prelude::*;
use std::path::Path;

mod parse_torrent;
use parse_torrent::Torrent;

fn parser(buffer: &Vec<u8>) {
     match de::from_bytes::<Torrent>(&buffer) {
        Ok(t) => t.render(),
        Err(e) => println!("PARSE ERROR: {:?}", e),
    }
}

fn main() {
    let stdin = io::stdin();
    let mut buffer = Vec::new();
    let mut handle = stdin.lock();
    match handle.read_to_end(&mut buffer) {
        Ok(_) => parser(&buffer),
        Err(e) => println!("READ ERROR: {:?}", e),
    }
}

// TODO: Write Tests
/*
#[cfg(test)]
mod tests {
    #[test]
    fn test_torrents() {
        let file_names = read_dir("./tests").unwrap();

        for file_name in file_names {
            let mut file = File::open(file_name).unwrap();
            let mut buffer = Vec::new();

            parser();
        }
    }
}
*/
