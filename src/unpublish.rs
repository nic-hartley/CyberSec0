use std::path::Path;

pub fn unpublish(assets: &Path, id: &str) {
  let id = format!("{}.md", id);
  let from_path = assets.join("posts").join(&id);
  let to_path = assets.join("drafts").join(&id);
  std::fs::create_dir_all(to_path.parent().unwrap()).unwrap();
  std::fs::rename(from_path, to_path).unwrap();
}
