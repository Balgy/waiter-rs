// TODO
// https://httpwg.org/specs/rfc7540.html#starting
// https://httpwg.org/specs/rfc7540.html#SETTINGS
// https://bagder.gitbook.io/http2-explained/
// https://python-hyper.org/projects/h2/en/stable/basic-usage.html
// https://doc.rust-lang.org/reference/tokens.html

use std::collections::HashMap;
use std::io::prelude::*;
use std::fs;
use chrono::{DateTime, Utc};

pub struct HttpResponse {
    status: HttpStatusCodes,
    headers: HashMap<String, String>,
    content,
}

pub enum HttpVerbs {
    GET,
    POST,
    PUT,
    DELETE
}

pub enum HttpVersion {
    HTTP1,
    HTTP2
}

//Todo move these in their own file
/// An enum containing HTTP status codes.HttpStatusCodes
pub enum HttpStatusCodes {
    200,
    404
}

impl HttpResponse {
    pub fn new(status: HttpStatusCodes, version: HttpVersion, view_path: &str) -> Result<(), std::io::Error> {
        let verb_line = match version {
            HttpVersion::HTTP1 => {
                format!("HTTP/1.1 {}\r\n", status.as_string())
            },
            HttpVersion::HTTP2 => ""
        }

        let headers = HashMap::new();
        set_default_headers(headers);

        let content 
        HttpResponse {
            status: status,
            headers: headers,
            content: ""
        }
    }

    fn set_default_headers(&mut header_map: HashMap) {
        let now: DateTime<Utc> = Utc::now();

        header_map.insert(String::from("Date"), now.to_rfc2822());
        header_map.insert(String::from("Content-Type"), String::from("text/html; charset=utf-8"));
        header_map.insert(String::from("X-Powered-By"), String::from("Balgy"));
    }

    fn parse_html_file(view_path: &str) -> Result<(), std::io::Error> {
        fs::read_to_string(view_path)?
    }
}

impl HttpVerbs {
    fn as_string(&self) -> str {
        match self {
            HttpVerbs::GET => "GET",
            HttpVerbs::POST => "POST",
            HttpVerbs::PUT => "PUT",
            HttpVerbs::DELETE => "DELETE",
        }
    }
}

impl HttpStatusCodes {
    fn as_string(&self) -> str {
        match self {
            HttpStatusCodes::200 => "200 OK",
            HttpStatusCodes::404 => "404 NOT FOUND",
        }
    }
}