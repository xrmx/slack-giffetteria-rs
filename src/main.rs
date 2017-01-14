extern crate curl;
extern crate env_logger;
extern crate futures;
extern crate regex;
extern crate rustc_serialize;
extern crate tokio_core;
extern crate tokio_curl;
extern crate tokio_minihttp;
extern crate tokio_proto;
extern crate tokio_service;

use std::io::{self, Write};
use std::str;

use curl::easy::Easy;
use futures::future;
use regex::Regex;
use rustc_serialize::json;
use tokio_core::reactor::Core;
use tokio_curl::Session;
use tokio_minihttp::{Request, Response, Http};
use tokio_proto::TcpServer;
use tokio_service::Service;

#[derive(RustcDecodable)]
struct SlackMessage {
	token: String,
	team: String,
	team_domain: String,
	channel_id: String,
	channel_name: String,
	user_id: String,
	user_name: String,
	command: String,
	text: String,
	response_url: String,
}

#[derive(RustcEncodable)]
struct SlackResponseAttachments {
	text: String
}

#[derive(RustcEncodable)]
struct SlackResponse {
	text: String,
	attachments: Vec<SlackResponseAttachments>
}

struct Giffetteria;

impl Service for Giffetteria {
	type Request = Request;
	type Response = Response;
	type Error = io::Error;
	type Future = future::Ok<Response, io::Error>;

	fn call(&self, request: Request) -> Self::Future {
		let mut resp = Response::new();
		/*let mut decoded: SlackMessage;
		match json::decode(request.data()) {
			Ok(msg) => decoded = msg,
			Err(msg) => {
				resp.body("400 Bad request");
				resp.status_code(400, "Bad request");
				return future::ok(resp);
			}
		}*/

		let mut lp = Core::new().unwrap();
        let session = Session::new(lp.handle());
        // FIXME: url encoding
        //let search_url = format!("http://giffetteria.it/?s={}", decoded.text.to_owned()).as_str();

        let mut response_content: &'static str;
        let mut req = Easy::new();
        req.get(true).unwrap();
        req.url("http://giffetteria.it/?s=prova").unwrap();
        req.write_function(|data| {
            response_content = str::from_utf8(&data).unwrap();
            Ok(data.len())
        }).unwrap();

        let r = session.perform(req);
        let result = lp.run(r).unwrap();
        /*
        if result.response_code().unwrap() != 200 {
				resp.body("404 Not found");
				resp.status_code(404, "Not found");
				return future::ok(resp);
        }
        let re = Regex::new(r#"data-gif=".*""#).unwrap();
        let mut title: String;
		let mut gif_url: String;

        //FIXME: collect and pick a random one
        for gif in re.matches(result.data).into_iter() {
            title = gif;
            gif_url = gif;
            break;
        }
		let object = SlackResponse {
			text: title,
			attachments: vec![
				SlackResponseAttachments {
					text: gif_url
				}
			]
		};
		let encoded: String = json::encode(&object).unwrap();
		resp.body(encoded.as_str());
		future::ok(resp)
	    */
        resp.body("");
        future::ok(resp)
    }
}

fn main() {
	drop(env_logger::init());
	let addr = "0.0.0.0:8888".parse().unwrap();
	TcpServer::new(Http, addr)
		.serve(|| Ok(Giffetteria));
}
