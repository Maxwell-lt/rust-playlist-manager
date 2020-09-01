use std::path::Path;
use std::fs::ReadDir;
use std::fs::DirEntry;
use colored::*;

fn get_file_list() -> ReadDir {
  Path::new(".").read_dir().expect("Could not read directory contents")
}

fn get_file_extension(file: DirEntry) -> Option<String> {
  let path = file.path();
  match path.extension() {
    Some(e) => e.to_str().map(String::from),
    None => None
  }
}

fn main() {
  for file_result in get_file_list() {
    let file = file_result.expect("Invalid file");
    println!("File: {}", file.file_name().to_str().expect("Invalid string").blue());
    let extension = get_file_extension(file);
    match extension {
      Some(e) => println!("Extension: {}", e.blue()),
      _ => println!("{}", "No extension".red()),
    }
    println!("");
  }
}
