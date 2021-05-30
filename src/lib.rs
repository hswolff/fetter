use std::fs;
use std::io;
use std::path::Path;

pub fn read_dir(dir: &Path) -> io::Result<()> {
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
