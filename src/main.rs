use make_magic::magic;
use traits::DasHash;

mod make_magic;
mod traits;
fn main() {
    println!("{}", "acb".hash());
    println!("{}", "torogarano".hash());
    println!("{}", "c".hash());
    println!("{}", "d".hash());
    println!("{}", "e".hash());
}
