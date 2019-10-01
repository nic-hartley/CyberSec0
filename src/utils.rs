use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead as _, Read as _, Write as _},
    path::Path,
};

extern crate pulldown_cmark;
use pulldown_cmark::{html, Options, Parser};

// I'd normally return an `io::Read`, but my Markdown library only takes &str
// also, "HMD" stands for "header'd markdown" -- see assets/blog for examples
pub fn parse_hmd_file(path: &Path) -> (HashMap<String, String>, String) {
    let mut input = io::BufReader::new(fs::File::open(path).unwrap());
    let header = {
        let mut header = HashMap::new();
        let mut line = String::new();
        while input.read_line(&mut line).unwrap() > 0 {
            if line == "---\n" {
                break;
            }
            if !line.starts_with("> ") {
                panic!("No header section?!");
            } else {
                line = line.split_off(2).trim_end().into();
            }
            let colon = line.find(": ").unwrap();
            let (name, val) = line.split_at(colon);
            let val = val.to_owned().split_off(2);
            header.insert(name.into(), val);
            line.clear();
        }
        header
    };
    let mut body = String::new();
    input.read_to_string(&mut body).unwrap();
    let body = html_from_md(body);
    (header, body)
}

fn html_from_md(md: String) -> String {
    let mut out = String::new();
    html::push_html(&mut out, Parser::new_ext(&md, Options::all()));
    out
}
