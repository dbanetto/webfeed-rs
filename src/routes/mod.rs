use config::Config;

use hbs::Template;
use rustc_serialize::json::{Json, ToJson};
use iron::prelude::*;
use std::io::prelude::*;
use std::str::FromStr;
use iron::status;
use rss::Channel;
use hyper::client::Client;

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
    let mut channels = vec![];

    let config = req.extensions.get::<Config>().unwrap();
    let client = Client::new();

    for channel in config.channels() {
        let req = client.get(&channel.url);

        let returned = req.send();
        match returned {
            Ok(mut res) => {
                let mut body = String::new();
                let _ = res.read_to_string(&mut body);
                let channel = Channel::from_str(&body).unwrap();

                channels.push(channel);
                // let mut history = BTreeMap::new();
                // history.insert("", rss.
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    let items: Vec<_> = channels.into_iter().flat_map(|c| c.items).collect();

    println!("{:?}", items);

    let rss_result = Channel {
        title: "".to_owned(),
        link: "/rss".to_owned(),
        description: "".to_owned(),
        items: items,
        ..Default::default()
    };
    let mut res = Response::with((status::Ok, rss_result.to_string()));
    res.headers.set_raw("Content-Type", vec![b"application/xml".to_vec()]);
    Ok(res)
}
