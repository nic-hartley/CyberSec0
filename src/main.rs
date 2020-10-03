use std::path::Path;

extern crate walkdir;
extern crate askama;
extern crate rsass;
extern crate chrono;

mod write_adapter;
mod utils;
mod render;
mod publish;
mod unpublish;

fn main() {
    let root = Path::new(".");
    let assets = root.join("assets");
    let out = root.join("docs");
    
    let mut args: Vec<_> = std::env::args().collect();
    let called_as = args.remove(0);
    if args.len() == 0 || args[0] == "render" {
        let begin = std::time::Instant::now();
        render::render(&assets, &out);
        let end = std::time::Instant::now();
        println!("Rendering took {}ms", (end - begin).as_millis());
    } else if args[0] == "publish" && args.len() == 2 {
        let begin = std::time::Instant::now();
        publish::publish(&assets, &args[1]);
        render::render(&assets, &out);
        let end = std::time::Instant::now();
        println!("Publishing took {}ms", (end - begin).as_millis());
    } else if args[0] == "unpublish" && args.len() == 2 {
        let begin = std::time::Instant::now();
        unpublish::unpublish(&assets, &args[1]);
        render::render(&assets, &out);
        let end = std::time::Instant::now();
        println!("Publishing took {}ms", (end - begin).as_millis());
    } else {
        println!(concat!(
            "Usage: \n",
            "  {0} [render]\n",
            "    Re-generate the static website.\n",
            "  {0} publish <id>\n",
            "    Publish the post with the given ID.\n",
            "  {0} unpublish <id>\n",
            "    Take the post off the website.\n",
            "Publishing and unpublishing automatically re-render the site as well."
        ), called_as);
    }
}
