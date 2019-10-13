use std::{fs, io::Write as _, path::Path};

extern crate walkdir;
use walkdir::WalkDir;

extern crate askama;
use askama::Template;

extern crate rsass;

mod write_adapter;
use write_adapter::adapt;

mod utils;
use utils::*;

#[derive(Debug)]
struct Bio {
    id: String,
    name: String,
    email: String,
    site: String,
    role: String,
    body: String,
}

fn get_bios(assets: &Path) -> Vec<Bio> {
    let mut authors = vec![];
    for bio_file in fs::read_dir(assets.join("bios")).unwrap() {
        let bio_file = bio_file.unwrap().path();
        let id = bio_file.file_stem().unwrap().to_str().unwrap().into();
        let (mut props, body) = parse_hmd_file(&bio_file);
        authors.push(Bio {
            id,
            name: props.remove("name").unwrap(),
            email: props.remove("email").unwrap(),
            site: props.remove("site").unwrap(),
            role: props.remove("role").unwrap(),
            body,
        });
    }
    authors
}

#[derive(Debug)]
struct Post<'a> {
    id: String,
    title: String,
    author: &'a Bio,
    tags: Vec<String>,
    body: String,
    // TODO: `created` date automatically somehow?
}

fn get_posts<'a>(assets: &Path, authors: &'a [Bio]) -> Vec<Post<'a>> {
    let mut posts = vec![];
    for post_file in fs::read_dir(assets.join("blog")).unwrap() {
        let post_file = post_file.unwrap().path();
        let id = post_file.file_stem().unwrap().to_str().unwrap().into();
        let (mut props, body) = parse_hmd_file(&post_file);
        let author_id = props.remove("author").unwrap();
        let author = authors.iter().find(|a| a.id == author_id).unwrap();
        posts.push(Post {
            id,
            title: props.remove("title").unwrap(),
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

fn compile_styles(assets: &Path, out: &Path) {
    let compiled = rsass::compile_scss_file(
        &assets.join("styles").join("styles.scss"),
        rsass::OutputStyle::Compressed,
    )
    .expect("Failed to compile SCSS");
    let mut of =
        fs::File::create(out.join("styles.css")).expect("Failed to open file");
    of.write_all(&compiled).expect("Failed to write to file");
}

#[derive(Template)]
#[template(path = "site_root.html")]
struct SiteRootPage;

#[derive(Template)]
#[template(path = "blog_index.html")]
struct BlogIndexPage<'a, 'b> {
    posts: &'a [Post<'b>],
}

#[derive(Template)]
#[template(path = "post.html")]
struct PostPage<'a> {
    post: Post<'a>,
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutPage<'a> {
    bios: &'a [Bio],
}

#[derive(Template)]
#[template(path = "bio.html")]
struct BioPage {
    bio: Bio,
}

fn write_exact<T: askama::Template>(template: T, path: &Path) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let output = fs::File::create(path).unwrap();
    template.render_into(&mut adapt(output)).unwrap();
}

fn write<T: askama::Template>(template: T, path: &Path) {
    write_exact(template, &path.join("index.html"))
}

fn main() {
    let begin = std::time::Instant::now();

    let root = Path::new(".");
    let assets = root.join("assets");
    let out = root.join("docs");
    fs::create_dir_all(&out).unwrap();
    fs::remove_dir_all(&out).unwrap();
    fs::create_dir(&out).unwrap();

    let bios = get_bios(&assets);
    let posts = get_posts(&assets, &bios);

    copy_statics(&assets, &out);
    compile_styles(&assets, &out);
    write(SiteRootPage, &out);
    write(BlogIndexPage { posts: &posts }, &out.join("blog"));
    for post in posts.into_iter() {
        let output_path = out.join("blog").join(&post.id);
        write(PostPage { post }, &output_path);
    }
    write(AboutPage { bios: &bios }, &out.join("about"));
    for bio in bios.into_iter() {
        let output_path = out.join("bios").join(&bio.id);
        write(BioPage { bio }, &output_path);
    }

    let end = std::time::Instant::now();
    println!("Generation took {}us", (end - begin).as_micros());
}
