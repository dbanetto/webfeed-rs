extern crate webfeed;

use webfeed::WebFeed;

fn main() {
    WebFeed::new().start("localhost:3000");
}
