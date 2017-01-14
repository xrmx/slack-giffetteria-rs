extern crate env_logger;
extern crate futures;
extern crate rustc_serialize;
extern crate tokio_minihttp;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;

use futures::future;
use rustc_serialize::json;
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

struct Giffetteria;

impl Service for Giffetteria {
	type Request = Request;
	type Response = Response;
	type Error = io::Error;
	type Future = future::Ok<Response, io::Error>;

	fn call(&self, request: Request) -> Self::Future {
		let mut resp = Response::new();
		let decoded: SlackMessage = json::decode(request.data.as_slice()).unwrap();
		// FIXME: curl
		// FIXME: encode response
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
