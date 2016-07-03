use iron::prelude::*;
use iron::middleware::AfterMiddleware;
use iron::status;
use rustc_serialize::json::{Json, ToJson};
use hbs::Template;

use std::collections::BTreeMap;
use std::error::Error as StdError;

pub struct ErrorHandler;

impl ErrorHandler {
    pub fn new() -> Self {
        ErrorHandler
    }
}

impl AfterMiddleware for ErrorHandler {
    fn after(&self, _: &mut Request, res: Response) -> IronResult<Response> { Ok(res) }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        let description = err.description().to_owned();
        let mut res = err.response;
        let mut data = BTreeMap::<String, Json>::new();
        data.insert("error".to_owned(), description.to_json());
        res.set_mut(Template::new("error", data));
        Ok(res)
    }
}
