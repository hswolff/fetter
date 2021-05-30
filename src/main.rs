use fetter;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    println!("Hello, world!");

    fetter::read_dir(Path::new("example"))?;

    Ok(())
}
