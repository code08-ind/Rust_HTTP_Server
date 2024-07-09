use super::StatusCode;
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};
use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

#[derive(Debug)]
pub struct Response{
    status_code: StatusCode,
    body: Option<String>,

}

impl Response{
    pub fn new(status_code: StatusCode, body: Option<String>)-> Self{
        Response{status_code, body}
    }

    // dyn: Dynamic
    // pub fn send_TcpStream(&self, stream:  &mut TcpStream)-> IoResult<()>{
    // pub fn send_File(&self, stream:  &mut File)-> IoResult<()>{

    pub fn send(&self, stream:  &mut impl Write)-> IoResult<()>{
        let body = match &self.body{
            Some(b) => b,
            None => ""
        };

        write!(stream, "HTTP/1.1 {} {}\r\n\r\n", self.status_code, self.status_code.reason_phrase(), body);
    }
}