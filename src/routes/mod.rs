use hbs::Template;
use std::collections::BTreeMap;

use rustc_serialize::json::{Json, ToJson};
use iron::prelude::*;
use iron::status;

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
