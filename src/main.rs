extern crate hyper;
extern crate futures;

use futures::future::Future;
use hyper::header::ContentLength;
use hyper::header::ContentType;
use hyper::server::{Http, Request, Response, Service};

#[derive(Clone)]
struct HelloWorld {
    phrase: String
}

impl Service for HelloWorld {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Pesponse your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future {
        // We're currently igonoring the Request
        // And returning an 'ok' Future, which means it's ready
        // immediately, and build a Response with the 'phrase' body.
        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(self.phrase.len() as u64))
                .with_header(ContentType::plaintext())
                .with_body(self.phrase.clone())))
    }
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let service = HelloWorld {phrase: String::from("今日は")};
    let server = Http::new().bind(&addr, move || Ok(service.clone())).unwrap();
    server.run().unwrap();
}
