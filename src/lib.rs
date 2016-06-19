extern crate iron;
extern crate handlebars_iron as hbs;
extern crate handlebars;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate rustc_serialize;
extern crate env_logger;

use iron::prelude::*;
use iron::status;
use router::Router;
use staticfile::Static;
use mount::Mount;
use hbs::{Template, HandlebarsEngine, DirectorySource};
use handlebars::Handlebars;
use rustc_serialize::json::{Json, ToJson};

#[cfg(feature = "watch")]
use hbs::Watchable;

use std::path::Path;
use std::collections::BTreeMap;
use std::error::Error;
use std::sync::Arc;

fn index(req: &mut Request) -> IronResult<Response> {
    println!("URL path: {:?}", req.url.path);
    let mut res = Response::new();
    let data = BTreeMap::<String, Json>::new();
    res.set_mut(Template::new("index", data))
        .set_mut(status::Ok);
    Ok(res)
}
fn error_404(_: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();
    let mut data = BTreeMap::<String, Json>::new();
    data.insert("error".to_owned(), "Page not found".to_json());
    res.set_mut(Template::new("error", data))
        .set_mut(status::NotFound);
    Ok(res)
}

#[cfg(feature = "watch")]
fn watch_hbs(hbs: Arc<HandlebarsEngine>) -> Arc<HandlebarsEngine> {
    hbs.watch("views");
    hbs
}

#[cfg(not(feature = "watch"))]
fn watch_hbs(hbs: Arc<HandlebarsEngine>) -> Arc<HandlebarsEngine> {
    hbs
}

pub struct WebFeed;

impl WebFeed {
    pub fn new() -> Self {
        WebFeed
    }
    
    pub fn start(&self, host: &str) {
        env_logger::init().unwrap();
        let mut hb = Handlebars::new();
        hb.register_template_file("layout", &Path::new("views/layout.hbs"));
        let mut hbse = HandlebarsEngine::from(hb);

        hbse.add(Box::new(DirectorySource::new("views", ".hbs")));

        // load templates from all registered sources
        if let Err(r) = hbse.reload() {
            panic!("{:?}", r);
        }

        let mut router = Router::new();
        router.get("/", index);
        router.any("**", error_404);

        let mut mount = Mount::new();
        mount.mount("/public/", Static::new(Path::new("public")))
            .mount("/", router);


        let mut chain = Chain::new(mount);
        chain.link_after(watch_hbs(Arc::new(hbse)));

        println!("Server running at {}", host);
        Iron::new(chain).http(host).unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
