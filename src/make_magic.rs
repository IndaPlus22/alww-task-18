use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use crate::traits::DasHash;

pub fn magic() {
    //sakao-kun
    let mut buf = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(File::open("token").unwrap()),
    );

    let mut line = String::new();
    buf.read_line(&mut line).unwrap();

    let mut dindong: Vec<&str> = Vec::new();
    let prev_key = 0;
    loop {
        let line: Vec<&str> = line.trim().split_ascii_whitespace().into_iter().collect();
        let item = line[0];
        let key = item.hash();
        let offset = line[1];
        if prev_key == key {
        } else {
            dindong = Vec::new();
            dindong.push(item);
            dindong.push(offset);
        }
    }
}
