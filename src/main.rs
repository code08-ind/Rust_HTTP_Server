#![allow(dead_code)]

use server::Server;
use http::Method;
use http::Request;
use website_handler::WebsiteHandler;
use std::env;

mod http;
mod server;
mod website_handler;

fn main() {
    // let string = String::from();

    // let string_slice = &string[10..];
    // let string_borrow : &str = &string;
    // let string_literal ="1234";

    // dbg!(&string);
    // dbg!(string_slice);
    // dbg!(string_borrow);
    // dbg!(string_literal);
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path =  env::var("PUBLIC_PATH").unwrap_or(default_path);
    print!("public path:{}", public_path);
    let get = Method::GET("abcd".to_string());
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}