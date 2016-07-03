use config::Config;

use hbs::Template;
use rustc_serialize::json::{self, Json, ToJson};
use iron::prelude::*;
use iron::status;

use std::collections::BTreeMap;

pub fn index(req: &mut Request) -> IronResult<Response> {
    println!("URL path: {:?}", req.url.path);
    let mut res = Response::new();
    let data = BTreeMap::<String, Json>::new();
    res.set_mut(Template::new("index", data))
        .set_mut(status::Ok);
    Ok(res)
}

pub fn error_404(_: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();
    let mut data = BTreeMap::<String, Json>::new();
    data.insert("error".to_owned(), "Page not found".to_json());
    res.set_mut(Template::new("error", data))
        .set_mut(status::NotFound);
    Ok(res)
}

pub fn rss(req: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();
    let mut data = BTreeMap::<String, Json>::new();
    let config = req.extensions.get::<Config>().unwrap();

    data.insert("config".to_owned(), config.to_json());
    println!("{:?}", data);

    res.set_mut(Template::new("rss", data))
       .set_mut(status::Ok);
    Ok(res)
}
