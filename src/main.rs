use std::{
    fs,
    path::Path,
};

extern crate walkdir;
use walkdir::WalkDir;

mod write_adapter;
use write_adapter::adapt;

mod utils;
use utils::*;

mod templates;

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

fn get_posts(assets: &Path) -> Vec<templates::Post> {
    let mut posts = vec![];
    for post_file in fs::read_dir(assets.join("blog")).unwrap() {
        let post_file = post_file.unwrap().path();
        let id = post_file.file_stem().unwrap().to_str().unwrap().into();
        let (props, body) = parse_hmd_file(&post_file);
        posts.push(templates::Post {
            id,
            title: props["title"].clone(),
            author: props["author"].clone(),
            tags: props["tags"].split(',').map(Into::into).collect(),
            body,
        });
    }
    posts
}

fn write_template<T: askama::Template>(template: T, path: &Path) {
    fs::create_dir_all(path).unwrap();
    let path = path.join("index.html");
    let output = fs::File::create(path).unwrap();
    template.render_into(&mut adapt(output)).unwrap();
}

fn main() {
    let begin = std::time::Instant::now();

    let root = Path::new(".");
    let assets = root.join("assets");
    let out = root.join("docs");
    fs::create_dir_all(&out).unwrap();
    fs::remove_dir_all(&out).unwrap();
    fs::create_dir(&out).unwrap();

    let posts = get_posts(&assets);

    copy_statics(&assets, &out);
    write_template(templates::SiteRootPage, &out);
    write_template(templates::BlogIndexPage { posts: &posts }, &out.join("blog"));
    for post in posts.into_iter() {
        let output_path = out.join("blog").join(&post.id);
        write_template(templates::PostPage { post }, &output_path);
    }

    let end = std::time::Instant::now();
    println!("Generation took {}us", (end - begin).as_micros());
}
