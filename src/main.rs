use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let root_path = Path::new("example");

    // Since your main app returns the io::Result, you can just throw it directly from the function
    fetter::Fetter::create(root_path)
}
