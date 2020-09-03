use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::fs::ReadDir;
use std::fs::DirEntry;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;
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

fn get_file_name(path: &PathBuf) -> Option<String> {
  path.file_name()
    .and_then(|f| f.to_str())
    .map(String::from)
}

fn read_playlist(playlist: &PathBuf) -> Vec<PathBuf> {
  let mut playlist_paths: Vec<PathBuf> = Vec::new();
  let file = File::open(playlist);
  if let Ok(f) = file {
    let reader = BufReader::new(&f);
    for line in reader.lines() {
      if let Ok(text) = line {
        playlist_paths.push(Path::new(&text).to_path_buf());
      }
    }
  }
  playlist_paths
}

fn read_playlists(file_listing: &Vec<PathBuf>) -> HashMap<PathBuf, Vec<PathBuf>> {
  let mut playlist_map: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
  for playlist in file_listing {
    let paths = read_playlist(&playlist);
    playlist_map.insert(playlist.to_path_buf(), paths);
  }
  playlist_map
}

fn main() {
  let playlist_files = get_m3u_files(get_file_list());
  let playlist_data = read_playlists(&playlist_files);

  for (filename, filelist) in &playlist_data {
    println!("{}", get_file_name(filename).unwrap().blue());
    for file in filelist {
      match file.exists() {
        true => println!("\t{}", file.to_str().unwrap().cyan()),
        false => println!("\t{}", file.to_str().unwrap().red()),
      }
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn get_filename() {
    let file_path = "dir1/dir2/filename.ext";
    if let Some(file_name) = get_file_name(&Path::new(file_path).to_path_buf()) {
      assert!(file_name == "filename.ext");
    } else {
      assert!(false);
    }
  }

  #[test]
  fn get_playlists() {
    let dir = tempfile::tempdir().expect("Could not instantiate test dir");
    let path = dir.path();
    File::create(path.join("playlist1.m3u")).unwrap();
    File::create(path.join("playlist2.m3u")).unwrap();
    let playlists = get_m3u_files(path.read_dir().expect("Could not read temp dir"));
    assert_eq!(playlists.len(), 2);
  }
}
