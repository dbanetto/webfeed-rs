extern crate clap;
extern crate webfeed;

use webfeed::WebFeed;

use clap::{App, Arg};

fn main() {
    let matches = App::new("webfeed")
                .version("")
                .author("")
                .arg(Arg::with_name("binding")
                        .short("b")
                        .long("binding")
                        .help("Binds WebFeed to the specified location")
                        .takes_value(true)
                     )
                .get_matches();

    let binding = matches.value_of("binding").unwrap_or("localhost:3000");

    WebFeed::new().start(binding);
}
