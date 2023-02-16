use std::{
    error::Error,
    fs::OpenOptions,
    io::{self, BufRead, Read},
};

use crate::traits::DasHash;

pub fn search(input: &str) -> Result<(), Box<dyn Error>> {
    let key = input.to_lowercase().hash();
    let mut path = ".files/indexes/".to_owned();
    path.push_str(&key.to_string());
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap();

    let mut csv = String::new();
    file.read_to_string(&mut csv);

    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    for node in reader.deserialize() {
        let node: Vec<String> = node?;
        println!("Match!\nFound {} matches.", node.len());
        let mut bool_print = false;
        if node.len() > 25 {
            println!("Would you like to print all {} matches?(Y/N)", node.len());
            let input = io::stdin();
            let mut lines = input.lock().lines();
            bool_print = match lines.next().unwrap().unwrap().to_lowercase().as_str() {
                "y" => true,
                _ => false,
            };
        }
        if (!bool_print) {
            break;
        }
        for offset in &node {}
    }
    Ok(())
}
