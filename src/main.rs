use std::env;

use lib_search::search;
use make_magic::magic;
use traits::DasHash;

mod lib_search;
mod make_magic;
mod traits;
fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = &args[1];
    let mut param = &String::new();
    if arg.as_str() != "compile" {
        if args.len() < 3 {
            panic!("\n\nError: missing or invalid arguments\n\n",);
        }
        param = &args[2];
    }

    match arg.as_str() {
        "compile" | "add" => magic(),
        "search" => search(param.as_str()),
        _ => {
            panic!("Error: could not parse the argument\n\n");
        }
    }
}
