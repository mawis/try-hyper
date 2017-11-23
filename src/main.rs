extern crate hyper;
extern crate futures;

use futures::future::Future;
use futures::Stream;
use hyper::{Body, Chunk, Method, StatusCode};
use hyper::header::ContentLength;
use hyper::header::ContentType;
use hyper::server::{Http, Request, Response, Service};
use std::ascii::AsciiExt;

struct Echo;

impl Service for Echo {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response<Box<Stream<Item=Chunk, Error=Self::Error>>>;
    type Error = hyper::Error;
    // The future representing the eventual Pesponse your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();

        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                let body: Box<Stream<Item=_, Error=_>> =
                    Box::new(Body::from("Try POSTing to /echo!"));
                response.set_body(body);
            },
            (&Method::Post, "/echo") => {
                let mapping =
                    req.body().map(to_uppercase as fn(Chunk) -> Chunk);
                let body: Box<Stream<Item=_, Error=_>> = Box::new(mapping);
                response.set_body(body);
            },
            _ => {
                response.set_status(StatusCode::NotFound);
            }
        }

        Box::new(futures::future::ok(response))
    }
}

fn to_uppercase(chunk: Chunk) -> Chunk {
    Chunk::from(
        chunk.iter()
            .map(|byte| byte.to_ascii_uppercase())
            .collect::<Vec<u8>>())
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
    server.run().unwrap();
}
