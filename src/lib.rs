extern crate hyper;
extern crate iron;
extern crate handlebars;
extern crate handlebars_iron as hbs;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate rustc_serialize;
extern crate env_logger;
extern crate rss;

mod routes;
pub mod config;

use config::{Config, ConfigMiddware};

use iron::prelude::*;
use staticfile::Static;
use mount::Mount;
use hbs::{HandlebarsEngine, DirectorySource};
use handlebars::Handlebars;
use router::Router;

#[cfg(feature = "watch")]
use hbs::Watchable;

use std::path::Path;
use std::sync::Arc;


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
        hb.register_template_file("layout", &Path::new("views/layout.hbs")).unwrap();
        let mut hbse = HandlebarsEngine::from(hb);

        hbse.add(Box::new(DirectorySource::new("views", ".hbs")));

        // load templates from all registered sources
        if let Err(r) = hbse.reload() {
            panic!("{:?}", r);
        }

        let mut config = Config::new();
        config.add_channel("http://www.newrustacean.com/feed.xml".to_owned());

        let mut router = Router::new();
        router.get("/", routes::index);
        router.get("/rss", routes::rss);
        router.any("**", routes::error_404);

        let mut mount = Mount::new();
        mount.mount("/public/", Static::new(Path::new("public")))
            .mount("/", router);

        let mut chain = Chain::new(mount);
        chain.link_before(ConfigMiddware::new(config));
        chain.link_after(watch_hbs(Arc::new(hbse)));

        println!("Server running at {}", host);
        Iron::new(chain)
            .http(host)
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
}
