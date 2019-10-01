extern crate askama;
use askama::Template;

pub struct Post {
    pub id: String,
    pub title: String,
    pub author: String, // TODO: Get author info from bio
    pub tags: Vec<String>,
    pub body: String,
    // TODO: `created` date automatically somehow?
}

#[derive(Template)]
#[template(path = "site_root.html")]
pub struct SiteRootPage;

#[derive(Template)]
#[template(path = "blog_index.html")]
pub struct BlogIndexPage<'a> {
    pub posts: &'a [Post]
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostPage {
    pub post: Post,
}
