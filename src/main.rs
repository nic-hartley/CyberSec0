use std::{
    fs,
    path::Path,
};

extern crate walkdir;
use walkdir::WalkDir;

extern crate askama;
use askama::Template;

mod write_adapter;
use write_adapter::adapt;

mod utils;
use utils::*;

#[derive(Debug)]
struct Author {
    id: String,
    name: String,
    email: String,
    site: String,
    bio: String,
}

fn get_authors(assets: &Path) -> Vec<Author> {
    let mut authors = vec![];
    for bio_file in fs::read_dir(assets.join("bios")).unwrap() {
        let bio_file = bio_file.unwrap().path();
        let id = bio_file.file_stem().unwrap().to_str().unwrap().into();
        let (props, body) = parse_hmd_file(&bio_file);
        authors.push(Author {
            id,
            name: props["name"].clone(),
            email: props["email"].clone(),
            site: props["site"].clone(),
            bio: body,
        });
    }
    authors
}

#[derive(Debug)]
struct Post<'a> {
    id: String,
    title: String,
    author: &'a Author,
    tags: Vec<String>,
    body: String,
    // TODO: `created` date automatically somehow?
}

fn get_posts<'a>(assets: &Path, authors: &'a [Author]) -> Vec<Post<'a>> {
    let mut posts = vec![];
    for post_file in fs::read_dir(assets.join("blog")).unwrap() {
        let post_file = post_file.unwrap().path();
        let id = post_file.file_stem().unwrap().to_str().unwrap().into();
        let (props, body) = parse_hmd_file(&post_file);
        let author_id = &props["author"];
        let author = authors.iter().find(|a| &a.id == author_id).unwrap();
        posts.push(Post {
            id,
            title: props["title"].clone(),
            author: author,
            tags: props["tags"].split(',').map(Into::into).collect(),
            body,
        });
    }
    posts
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

#[derive(Template)]
#[template(path = "site_root.html")]
struct SiteRootPage;

#[derive(Template)]
#[template(path = "blog_index.html")]
struct BlogIndexPage<'a, 'b> {
    posts: &'a [Post<'b>]
}

#[derive(Template)]
#[template(path = "post.html")]
struct PostPage<'a> {
    post: Post<'a>,
}

fn write<T: askama::Template>(template: T, path: &Path) {
    fs::create_dir_all(path).unwrap();
    let path = path.join("index.html");
    let output = fs::File::create(path).unwrap();
    template.render_into(&mut crate::adapt(output)).unwrap();
}

fn main() {
    let begin = std::time::Instant::now();

    let root = Path::new(".");
    let assets = root.join("assets");
    let out = root.join("docs");
    fs::create_dir_all(&out).unwrap();
    fs::remove_dir_all(&out).unwrap();
    fs::create_dir(&out).unwrap();

    let authors = get_authors(&assets);
    let posts = get_posts(&assets, &authors);

    copy_statics(&assets, &out);
    write(SiteRootPage, &out);
    write(BlogIndexPage { posts: &posts }, &out.join("blog"));
    for post in posts.into_iter() {
        let output_path = out.join("blog").join(&post.id);
        write(PostPage { post }, &output_path);
    }

    let end = std::time::Instant::now();
    println!("Generation took {}us", (end - begin).as_micros());
}
