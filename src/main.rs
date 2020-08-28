use std::path::Path;
use colored::*;

fn main() {
    let path = Path::new(".");
    let files = path.read_dir().expect("Could not read directory contents");
    for file in files {
      match file {
        Err(_) => println!("Error in directory listing"),
        Ok(f) => match (*f.path()).is_file() {
          true => println!("{}", f.file_name().into_string().expect("").cyan()),
          false => println!("{}", f.file_name().into_string().expect("").blue()),
        }
      }
    }
}
