use std::path::Path;
use colored::*;

fn main() {
  let dir_list = Path::new(".")
                  .read_dir().expect("Could not read directory contents");
  let files = dir_list.filter_map(|file| 
  {
    match file {
      Ok(f) =>
        match f.path().is_file() {
          true => Some(f.path()),
          false => None,
        },
      Err(_) => None,                     
    }
  });
  for file in files {
    println!("{}",
      file.to_str().expect("Invalid string").blue());
  }
}
