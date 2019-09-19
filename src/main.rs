use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

struct Author {
  name: String,
  bio: String,
  bio_url: String,
}

struct Post {
  title: String,
  tags: Vec<String>,
  author: String,
  body: String,
  published: bool,
  url: String,
}

fn title_to_url(title: &str) -> String {
  let bad_char = |c: char| !c.is_ascii_alphanumeric() && c != '-';
  title.replace(" ", "-").replace(bad_char, "").to_ascii_lowercase()
}

fn read_post(path: &Path) -> Option<Post> {
  let file = fs::File::open(path).expect("Failed to read post {?:}", path);
  let file = BufReader::new(file);
  let lines = file.lines();
  for line in lines {
    
  }
  let mut body = String::new();
  
}

fn main() {
  // TODO arg parsing
  let cwd = Path::new(".").canonicalize().expect("Could not open .");
  let assets_path = cwd.join("./assets");
  let posts_path = assets_path.join("posts");
  let templates_path = assets_path.join("templates");
  let output_path = cwd.join("docs");

  let posts: Vec<_> = fs::read_dir(posts_path)
    .expect("Failed to open posts directory")
    .map(|op| op.expect("Failed to open post"))
    .filter_map(read_post)
    .collect();

  write_index(&output_path, &posts, &templates_path.join("index.html"));
  write_posts(&output_path, &posts);
  write_bios()
}
