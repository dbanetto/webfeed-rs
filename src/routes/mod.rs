use config::Config;

use hbs::Template;
use rustc_serialize::json::{self, Json, ToJson};
use iron::prelude::*;
use iron::status;

use std::collections::BTreeMap;

pub fn index(req: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();
    let mut data = BTreeMap::<String, Json>::new();
    let config = req.extensions.get::<Config>().unwrap();

    data.insert("config".to_owned(), config.to_json());

    res.set_mut(Template::new("index", data))
        .set_mut(status::Ok);
    Ok(res)
}


pub fn rss(req: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();
    let mut data = BTreeMap::<String, Json>::new();
    let config = req.extensions.get::<Config>().unwrap();

    data.insert("config".to_owned(), config.to_json());

    res.set_mut(Template::new("rss", data))
        .set_mut(status::Ok);
    Ok(res)
}
