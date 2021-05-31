use fetter;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let root_path = Path::new("example");
    fetter::Fetter::create(root_path)?;

    Ok(())
}
