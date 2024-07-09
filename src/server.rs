use std::io::{Read, Write};
use std::net::TcpListener;
use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::convert::TryInto;

pub trait Handler{
    fn handle_request(&self, request: &Request) -> Response;

    fn handle_bad_request(&self, e: &ParseError) -> Response{
        println!("Failed to parse a request:{}", e);
        Response::new(StatusCode::BadRequest, None);
    }
}

pub struct Server {
    addr: String,
}

// Now we don't always need the size to be passed inside it, we just need a reference here.
// fn arr(a:&[u8]){

// }

impl Server {
    // Here self is an alias for Server.
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(&mut self, mut handler: impl Handler){
        // self follows ownership rules for the variables.
        println!("Listening on {}", self.addr);

        // This is used to make connections to the TCP. It returns a result type that wraps the TCP listener. It is used for error handling here and is one of the most important type we will be using.
        let listener = TcpListener::bind(&self.addr);

        // 'outer: loop {
        //     loop{
        //         // We can break or continue outer loop. This is how we basically handle how we want to work with a particular loop by giv 
        //         break 'outer;
        //     }
        // }

        loop{ 
            match listener.accept(){
                // If we don't want anything here, we can do this by using the _ here in place of the parameters we are passing.
                Ok((mut stream, _)) => {
                    // We csn also pass the string slices in place of the normal array.
                    // We can also pass a buffer here with an initial value and the length of the array inside it in bytes.
                    // let a = [1,2,3,4, 5];

                    // Our buffer will be able to handle any request out here.
                    let mut buffer = [0, 1024];
                    match stream.read(&mut buffer){
                        Ok(_)=> {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]){
                                Ok(request) => {
                                //     dbg!(request);
                                //    Response::new(StatusCode::Ok, Some("<h1>It Works !!!</h1>".to_string()));
                                // }
                                    handler.handle_request(&request)
                                },
                                Err(e) =>{ 
                                //     println!("Failed to parse a request:{}", e);
                                // Response::new(StatusCode::BadRequest, None);
                                handler.handle_bad_request(&e)
                            }
                            };
                            if let Err(e) = response.send(&mut stream){
                                println!("Failed to send a response: {}", e);
                            }
                        }
                Err(e) =>{ println!("Failed to read from connection: {}", e);
                },
                Err(e) => {
                    println!("Failed to establish a connection: {:?}", e);
                }
            }

            let res = listener.accept();

            if res.is_err(){
                continue;
            }

            let (stream, addr) = res.unwrap();
        }
    }
}