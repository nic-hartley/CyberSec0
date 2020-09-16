use std::{fs, io::Write as _, path::Path, cmp};

extern crate walkdir;
use walkdir::WalkDir;

extern crate askama;
use askama::Template;

extern crate rsass;

extern crate chrono;
use chrono::{prelude::*, /* format::strftime */};

mod write_adapter;

mod utils;
use utils::*;

const DATE_FMT: &'static str = "%Y-%m-%d";
// const RFC_822_FMT: &'static str = "%a, %d %b %Y %H:%M:%S %Z";

#[derive(Debug)]
struct Post {
    id: String,
    title: String,
    category: Option<String>,
    publish: NaiveDate,
    body: String,
}

#[derive(Debug, Clone)]
struct Category {
    name: String,
    intro: String,
}

fn get_posts(dir: &Path) -> Vec<Post> {
    let mut posts = vec![];
    let today = Local::now().naive_local().date();
    for post_file in fs::read_dir(dir).unwrap() {
        let post_file = post_file.unwrap().path();
        let id = post_file.file_stem().unwrap().to_str().unwrap().into();
        let (mut props, body) = parse_hmd_file(&post_file);
        let publish = match props.remove("publish") {
            Some(s) => NaiveDate::parse_from_str(&s, DATE_FMT).unwrap(),
            None => continue,
        };
        if publish > today {
            println!("Post {} scheduled for {}, skipping", id, publish);
            continue;
        }
        posts.push(Post {
            id,
            title: props.remove("title").unwrap(),
            category: props.remove("category"),
            publish,
            body,
        });
    }
    posts.sort_by_key(|p| cmp::Reverse(p.publish));
    posts
}

fn get_categories(dir: &Path) -> Vec<Category> {
    let mut categories = vec![];
    for cat_file in fs::read_dir(dir).unwrap() {
        let cat_file = cat_file.unwrap().path();
        let name = cat_file.file_stem().unwrap().to_str().unwrap().into();

        let md_intro = fs::read_to_string(cat_file).unwrap();
        let intro = html_from_md(md_intro);

        categories.push(Category { name, intro });
    }
    categories
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
#[template(path = "global.html")]
struct GenericPage {
    title: String,
    body: String,
}

#[derive(Template)]
#[template(path = "category_index.html")]
struct CategoryIndex<'a> {
    category: Category,
    posts: Vec<&'a Post>,
}

#[derive(Template)]
#[template(path = "rss.xml")]
struct RssFeed<'a> {
    posts: &'a [Post],
    gen_time: NaiveDateTime,
}

#[derive(Template)]
#[template(path = "post.html")]
struct PostPage {
    post: Post,
}

fn main() {
    let begin = std::time::Instant::now();
    
    let root = Path::new(".");
    let assets = root.join("assets");
    let out = root.join("docs");
    
    let posts = get_posts(&assets.join("posts"));

    let categories = get_categories(&assets.join("categories"));

    fs::create_dir_all(&out).unwrap();
    fs::remove_dir_all(&out).unwrap();
    fs::create_dir(&out).unwrap();

    copy_statics(&assets, &out);
    compile_styles(&assets, &out);

    compile_md(&assets.join("home.md"), "Home", &out);
    compile_md(&assets.join("about.md"), "About", &out.join("about"));

    let blog_intro_md = fs::read_to_string(&assets.join("blog_intro.md")).unwrap();
    let blog_intro = html_from_md(blog_intro_md);

    write_exact(RssFeed { posts: &posts, gen_time: Local::now().naive_local() }, &out.join("rss.xml"));

    for category in categories {
        let out_path = out.join(&category.name);
        let filtered: Vec<_> = posts.iter().filter(|p| p.category.as_ref() == Some(&category.name)).collect();
        if filtered.is_empty() {
            println!("Empty category: {}", category.name);
        }
        write(CategoryIndex { category, posts: filtered }, &out_path);
    }
    write(CategoryIndex {
        category: Category { name: "blogposts".into(), intro: blog_intro },
        posts: posts.iter().collect(),
    }, &out.join("blog"));

    for post in posts.into_iter() {
        let out_path = out.join("posts").join(&post.id);
        write(PostPage { post }, &out_path);
    }

    let end = std::time::Instant::now();
    println!("Generation took {}ms", (end - begin).as_millis());
}
