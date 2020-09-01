use std::path::Path;
use std::path::PathBuf;
use std::fs::ReadDir;
use std::fs::DirEntry;
use colored::*;

fn get_file_list() -> ReadDir {
  Path::new(".").read_dir().expect("Could not read directory contents")
}

fn get_file_extension(file: &DirEntry) -> Option<String> {
  let path = file.path();
  match path.extension() {
    Some(e) => e.to_str().map(String::from),
    None => None
  }
}

fn get_m3u_files(files: ReadDir) -> Vec<PathBuf> {
  let mut m3u_files: Vec<PathBuf> = Vec::new();
  for file in files {
    if let Ok(f) = file {
      if let Some(ext) = get_file_extension(&f) {
        if ext == "m3u" {
          m3u_files.push(f.path());
        }
      }
    }
  }
  m3u_files
}

fn get_file_name(path: PathBuf) -> Option<String> {
  path.file_name()
    .and_then(|f| f.to_str())
    .map(String::from)
}

fn main() {
  let playlists = get_m3u_files(get_file_list());
  for pl in playlists.into_iter().map(get_file_name).collect::<Vec<_>>() {
    if let Some(filename) = pl {
      println!("{}", filename.blue());
    }
  }
}
