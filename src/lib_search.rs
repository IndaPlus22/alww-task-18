use std::{
    error::Error,
    fs::OpenOptions,
    io::{self, BufRead, Read, Seek, SeekFrom},
};

use encoding_rs_io::DecodeReaderBytes;

use crate::traits::DasHash;

pub fn search(input: &str) {
    let key = input.to_lowercase().hash();
    let mut path = ".files/indexes/".to_owned();
    path.push_str(&key.to_string());
    let mut ifile = OpenOptions::new().read(true).open(path).unwrap();
    let mut path = ".files/korpus".to_owned();
    let mut kfile = OpenOptions::new().read(true).open(path).unwrap();
    let mut csv = String::new();
    ifile.read_to_string(&mut csv);
    csv.truncate(csv.len() - 1);

    let mut vec: Vec<&str> = csv.split(',').collect();
    println!("Found {} matches.", vec.len());
    let mut bool_print = true;
    if vec.len() > 25 {
        println!("Would you like to print all {} matches?(Y/N)", vec.len());
        let input = io::stdin();
        let mut lines = input.lock().lines();
        bool_print = match lines.next().unwrap().unwrap().to_lowercase().as_str() {
            "y" => true,
            _ => false,
        };
    }
    if (bool_print) {
        let mut so: u64 = 0;
        let mut eo: u64 = 79506750;
        let mut c = 0;
        for offset in &vec {
            let mut clone_of = kfile.try_clone().unwrap();
            // eprintln!("{}", offset);
            let offset: u64 = offset.parse().unwrap();
            if offset > 30 {
                so = offset - 30;
            }
            if offset + input.len() as u64 + 30 < 79506750 {
                eo = offset + input.len() as u64 + 30
            }
            let mut read = vec![0u8; 0];
            clone_of.seek(SeekFrom::Start(so)).unwrap();
            clone_of.take(eo - so).read_to_end(&mut read).unwrap();

            let mut dest = bytes_to_string(&read);
            println!("{}", dest);
        }
    }
}

fn bytes_to_string(s: &[u8]) -> String {
    let output: String = s.iter().map(|&c| c as char).collect();
    return output.replace("\n", " ");
}
