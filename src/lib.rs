use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::Path;

mod config;
use crate::config::Config;

static TOML_FILENAME: &str = "fetter.toml";

pub struct Fetter;

impl Fetter {
    pub fn create(root_dir: &Path) -> io::Result<()> {
        let fetter_toml = root_dir.join(TOML_FILENAME);

        // Since you want to panic, you can do it more easily using assertions
        assert!(fetter_toml.exists(), "Fetter app not found");

        let config = Config::new(fetter_toml)?;
        println!("name: {}", config.name);

        read_dir(root_dir)?;
        Ok(())
    }
}

fn read_dir(dir: &Path) -> io::Result<()> {
    // Return a custom IO error when it's not a dir
    if !dir.is_dir() {
        return Err(io::Error::new(
            ErrorKind::Other,
            "The provided dir is not a dir!",
        ));
    }

    // Flatten will help you when getting an iterator of iterators
    // And we just move through the iters that are Ok!
    for entry in (fs::read_dir(dir)?).flatten() {
        let path = entry.path();

        // Let's use a match here just for the sake of having one, they are amazing!
        match path.is_dir() {
            true => read_dir(&path)?,
            false => {
                // read_to_string already returns a io:Result, don't need to handle it manually ;)
                let content = fs::read_to_string(path)?;
                println!("{:?}\n{}", entry, content)
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
