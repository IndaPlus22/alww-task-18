use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Read, Write},
};

use crate::traits::DasHash;

pub fn magic() {
    let mut buf = BufReader::new(File::open(".files/token").unwrap());
    let mut line: Vec<u8> = Vec::new();
    buf.read_until(b'\n', &mut line);
    let mut pline = decode_utf8_or_850(line, false);
    // buf.read_line(&mut line);

    let mut dindong: Vec<String> = Vec::new();
    let mut prev_key = String::new();
    loop {
        let parsed_line: Vec<&str> = pline.trim().split_ascii_whitespace().into_iter().collect();
        let item = fix_word(parsed_line[0]);
        let key = item.clone().to_lowercase();
        if prev_key.is_empty() {
            prev_key = key.clone();
        }
        let mut path = ".files/indexes/".to_owned();
        path.push_str(prev_key.as_str());
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
        line = Vec::new();
        let bytes = buf.read_until(b'\n', &mut line);
        pline = decode_utf8_or_850(line, false);
        if bytes.unwrap() == 0 {
            break;
        }
    }
}

// Got help from jblomlof's solution
// got to know that å,ä,ö are the same in token. In my case I change it to unicode chars
fn fix_word(word: &str) -> String {
    let mut fixed = String::new();

    for c in word.bytes() {
        if c == 189 {
            // println!("c: {}", c);
            fixed.pop();
            fixed.pop();
            fixed.push(228 as char); // adding 'ä'
        } else {
            fixed.push(c as char);
        }
    }

    fixed
}
fn decode_utf8_or_850(buf: Vec<u8>, utf8: bool) -> String {
    if utf8 {
        match String::from_utf8(buf.clone()) {
            Ok(lines) => lines,
            Err(_) => String::from_utf8_lossy(&buf).to_string(),
        }
    } else {
        let enc = encoding_rs::Encoding::for_label("latin1".as_bytes());
        let mut dec = encoding_rs_io::DecodeReaderBytesBuilder::new()
            .encoding(enc)
            .build(&buf[..]);
        let mut res = String::new();
        dec.read_to_string(&mut res).unwrap();
        res
    }
}
