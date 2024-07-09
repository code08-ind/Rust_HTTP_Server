use crate::http::request;

use super::method::{Method, MethodError};
use std::convert::TryForm;
use std::error::Error;
use std::fmt::{write, Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;
use super::QueryString;

#[derive(Debug)]    
struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: super::method::Method,
}

impl<'buf> Request<'buf> {
    // fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}
    pub fn path(&self) -> &str{
        &self.path
    }

    pub fn method(&self) -> &Method{
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString>{
        self.query_string.as_ref()
    }
}

impl<'buf> TryForm<&'buf [u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        match str::from_utf8(buf) {
            Ok(request) => {
                let request = Request::from(request);
                return Ok(request);
            }
            Err(_) => return Err(ParseError::InvalidEncoding),
        }
        match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
            Ok(request) => {
                let request = Request::from(request);
                return Ok(request);
            }
            Err(e) => return Err(e),
        }

        let request = str::from_utf8(buf)?;

        match get_next_word(request){
            Some((method, request)) => {},
            None => return Err(ParseError::InvalidRequest),
        }

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if(protocol != "HTTP/1.1") {
            return Err(ParseError::InvalidProtocol);
        }

        let method = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find('?'){
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }

        Ok(Self{
            path,
            query_string,
            method,
        })

        unimplemented!()
    }
}

fn get_next_word<'a, 'b>(request: &'a str, b: &'b str) -> Option<(&'a str, &'b str)> {
    // let mut iter = request.chars();
    // loop {
    //     let item = iter.next();
    //     match item{
    //         Some(c) => {}
    //         None => break,
    //     }
    // }

    for (i, c) in request.chars().enumerate() {
        if c==' ' {
            return Some((&request[..i], &request[i+1..]));
        }
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.message());
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.message());
    }
}

impl Error for ParseError {}

trait Encrypt {
    fn encrypt(&self) -> Self;
}

impl Encrypt for String {
    fn encrypt(&self) -> Self {
        unimplemented!()
    }
}

impl Encrypt for &[u8] {
    fn encrypt(&self) -> Self {
        unimplemented!()
    }
}
