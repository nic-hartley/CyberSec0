use std::{fs, path::Path};

extern crate askama;
use askama::Template;

extern crate walkdir;
use walkdir::WalkDir;

mod write_adapter;
use write_adapter::WriteAdapter as WA;

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

fn copy_statics(assets: &Path, out: &Path) {
    let root = assets.join("static");
    for entry in WalkDir::new(&root) {
        let entry = entry.unwrap();
        let from_path = entry.path();
        let to_path = out.join(from_path.strip_prefix(&root).unwrap());
        if from_path.is_dir() {
            fs::create_dir_all(to_path).unwrap();
        } else {
            fs::copy(from_path, to_path).unwrap();
        }
    }
}

fn write_index(root: &Path) {
    let index_out = fs::File::create(root.join("index.html")).unwrap();
    Index.render_into(&mut WA::adapt(index_out)).unwrap();
}

fn main() {
    let begin = std::time::Instant::now();

    let root = Path::new(".");
    let assets = root.join("assets");
    let out = root.join("docs");
    fs::create_dir_all(&out).unwrap();
    fs::remove_dir_all(&out).unwrap();
    fs::create_dir(&out).unwrap();

    copy_statics(&assets, &out);
    write_index(&out);

    let end = std::time::Instant::now();
    println!("Generation took {}us", (end - begin).as_micros());
}
