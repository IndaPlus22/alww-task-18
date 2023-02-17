use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
};

use crate::traits::DasHash;

pub fn magic() {
    //sakao-kun
    let mut buf = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(File::open(".files/token").unwrap()),
    );

    let mut line = String::new();
    buf.read_line(&mut line);

    let mut dindong: Vec<String> = Vec::new();
    let mut prev_key = 0;
    loop {
        let parsed_line: Vec<&str> = line.trim().split_ascii_whitespace().into_iter().collect();
        let item = parsed_line[0];
        let key = item.to_lowercase().hash();
        if prev_key == 0 {
            prev_key = key;
        }
        let mut path = ".files/indexes/".to_owned();
        path.push_str(&prev_key.to_string());
        let offset = parsed_line[1];
        if prev_key == key {
            dindong.push(offset.to_string());
        } else {
            let mut writer = csv::Writer::from_path(path).unwrap();
            writer.serialize(dindong);
            writer.flush();
            dindong = Vec::new();
            dindong.push(offset.to_string());
        }
        prev_key = key;
        line.clear();
        let bytes = buf.read_line(&mut line);
        match bytes {
            Ok(0) => break,
            Ok(_) => continue,
            Err(_) => panic!("Error"),
        }
    }
}
