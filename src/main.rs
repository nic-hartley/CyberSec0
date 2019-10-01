use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead as _, Read as _},
    path::Path,
};

extern crate askama;
use askama::Template;

extern crate walkdir;
use walkdir::WalkDir;

mod write_adapter;
use write_adapter::adapt;

mod utils;
use utils::*;

struct Post {
    id: String,
    title: String,
    author: String, // TODO: Get author info from bio
    tags: Vec<String>,
    body: String,
    // TODO: `created` date automatically somehow?
}

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

fn get_posts(assets: &Path) -> Vec<Post> {
    let mut posts = vec![];
    for post_file in fs::read_dir(assets.join("blog")).unwrap() {
        let post_file = post_file.unwrap().path();
        let id = post_file.file_stem().unwrap().to_str().unwrap().into();
        let (props, body) = parse_hmd_file(&post_file);
        posts.push(Post {
            id,
            title: props["title"].clone(),
            author: props["author"].clone(),
            tags: props["tags"].split(',').map(Into::into).collect(),
            body,
        });
    }
    posts
}

fn write_index(out: &Path, posts: &[Post]) {
    #[derive(Template)]
    #[template(path = "index.html")]
    struct IndexPage<'a> {
        posts: &'a [Post],
    }

    let index_out = fs::File::create(out.join("index.html")).unwrap();
    IndexPage { posts }
        .render_into(&mut adapt(index_out))
        .unwrap();
}

fn write_posts(out: &Path, posts: Vec<Post>) {
    #[derive(Template)]
    #[template(path = "post.html")]
    struct PostPage {
        post: Post,
    }

    for post in posts.into_iter() {
        let output_path = out.join("blog").join(&post.id).join("index.html");
        fs::create_dir_all(output_path.parent().unwrap()).unwrap();
        let output = fs::File::create(&output_path).unwrap();
        PostPage { post }.render_into(&mut adapt(output)).unwrap();
    }
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
    write_index(&out, &posts);
    write_posts(&out, posts);

    let end = std::time::Instant::now();
    println!("Generation took {}us", (end - begin).as_micros());
}
