# WebFeed

[![Build Status](https://travis-ci.org/zyphrus/webfeed-rs.svg?branch=master)](https://travis-ci.org/zyphrus/webfeed-rs)

Rust powered website to collate RSS feeds.

## Goals

The goals of this project is to:

 * Try using rust for a backend of a web service
 * Make a RSS feed combiner

## How to run

There are two ways to run the server:

 * `cargo run` - without auto-updating the templates

 * `cargo run --features watch` - with auto-updating templates

After running either method open [`http://localhost:3000/`](http://localhost:3000/) in your web browser.

If the application panics on startup, ensure you are running it from
the base directory with `views` and `public` in it.

## License

WebFeed is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
