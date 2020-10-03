use std::{fs::File, io::{Write as _, BufRead as _, BufReader}, path::Path};
use chrono::prelude::*;

pub fn publish(assets: &Path, id: &str) {
  let id = format!("{}.md", id);

  let from_path = assets.join("drafts").join(&id);
  let mut from_file = BufReader::new(File::open(&from_path).unwrap());

  let to_path = assets.join("posts").join(&id);
  std::fs::create_dir_all(to_path.parent().unwrap()).unwrap();
  let mut to_file = File::create(to_path).unwrap();

  let today = Local::today().naive_local();
  let today_line = format!("> publish: {}\n", today.format(crate::utils::DATE_FMT));

  let mut line = String::new();
  while from_file.read_line(&mut line).unwrap() > 0 {
    if line.starts_with("---") {
      to_file.write(today_line.as_bytes()).unwrap();
      to_file.write(b"---\n").unwrap();
    } else if !line.starts_with("> ") {
      to_file.write(line.as_bytes()).unwrap();
      break;
    } else if line.starts_with("> publish:") {
      to_file.write(today_line.as_bytes()).unwrap();
      break;
    } else {
      to_file.write(line.as_bytes()).unwrap();
    }
    line.clear();
  }

  std::io::copy(&mut from_file, &mut to_file).unwrap();

  std::fs::remove_file(&from_path).unwrap();
}
