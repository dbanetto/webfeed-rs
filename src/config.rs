use rustc_serialize;
use rustc_serialize::json::{ToJson, Json};
use iron::prelude::*;
use iron::typemap::Key;
use iron::middleware::BeforeMiddleware;

use std::string::ToString;
use std::collections::BTreeMap;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Config {
    pub channels: Vec<Channel>,
}

impl Config {
    pub fn new() -> Self {
        Config { channels: vec![] }
    }

    pub fn add_channel(&mut self, url: String) {
        // FIXME: do not unwrap
        self.channels.push(Channel::new(url).unwrap());
    }
}

impl Key for Config {
    type Value = Config;
}

impl ToJson for Config {
    fn to_json(&self) -> Json {
        let mut obj = BTreeMap::new();
        obj.insert("channels".to_owned(), self.channels.to_json());

        Json::Object(obj)
    }
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Channel {
    url: String,
}

impl Channel {
    pub fn new<U: ToString>(url: U) -> Result<Self, ()> {
        Ok(Channel { url: url.to_string() })
    }
}

impl ToJson for Channel {
    fn to_json(&self) -> Json {
        let mut obj = BTreeMap::new();
        obj.insert("url".to_owned(), self.url.to_json());

        Json::Object(obj)
    }
}

pub struct ConfigMiddware {
    config: Config,
}

impl ConfigMiddware {
    pub fn new(config: Config) -> Self {
        ConfigMiddware {
            config: config,
        }
    }
}

impl BeforeMiddleware for ConfigMiddware {
    fn before(&self, req: &mut Request) -> IronResult<()> { 
        let config = self.config.clone();
        req.extensions.insert::<Config>(config);
        Ok(())
    }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<()> { 
        Err(err) 
    }
}
