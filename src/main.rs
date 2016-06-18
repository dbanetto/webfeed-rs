extern crate iron;
extern crate handlebars_iron as hbs;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use staticfile::Static;
use mount::Mount;
use hbs::{Template, HandlebarsEngine, DirectorySource};
use rustc_serialize::json::{Json, ToJson};

use std::path::Path;
use std::collections::BTreeMap;
use std::error::Error;

fn index(req: &mut Request) -> IronResult<Response> {
    println!("Running send_hello handler, URL path: {:?}", req.url.path);
    let mut res = Response::new();
    let data = BTreeMap::<String, Json>::new();
    println!("{:?}", data);
    res.set_mut(Template::new("layout", data)).set_mut(status::Ok);
    println!("wow {:?}", res);
    Ok(res)
}
fn error_404(_: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();
    let data = BTreeMap::<String, Json>::new();
    res.set_mut(Template::new("error", data)).set_mut(status::NotFound);
    Ok(res)
}

fn main() {
    let mut hbse = HandlebarsEngine::new();

    // hbse.register_template_file("template", "../views/layout.hbs");
    hbse.add(Box::new(DirectorySource::new("../views", ".hbs")));

    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{:?}", r);
    }

    let mut router = Router::new();
    router.get("/", index); 

    let mut mount = Mount::new();
    mount.mount("/public/", Static::new(Path::new("../public")))
         .mount("/", router);


    let mut chain = Chain::new(mount);
    chain.link_after(hbse);

    println!("Server running at http://localhost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}
