use std::fs;
use std::io;
use std::path::Path;
use std::process::exit;

mod config;
use crate::config::Config;

static TOML_FILENAME: &str = "fetter.toml";

pub struct Fetter {}

impl Fetter {
    pub fn create(root_dir: &Path) -> io::Result<()> {
        let fetter_toml = root_dir.join(TOML_FILENAME);

        if !fetter_toml.exists() {
            #[allow(unreachable_code)]
            {
                panic!("Fetter app not found");
                exit(1);
            }
        }

        let config = Config::new(fetter_toml)?;

        println!("name: {}", config.name);

        read_dir(root_dir)?;
        Ok(())
    }
}

fn read_dir(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                read_dir(&path)?;
            } else {
                let content = match fs::read_to_string(path) {
                    Ok(content) => content,
                    Err(_) => "Unable to parse".to_string(),
                };

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
