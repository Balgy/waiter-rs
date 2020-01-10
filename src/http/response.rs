use std::collections::HashMap;
use std::io::prelude::*;
use std::fs;
use chrono::{DateTime, Utc};
use common_types;

pub struct HttpResponse {
    status: HttpStatusCodes,
    headers: HashMap<String, String>,
    content,
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
        header_map.insert(String::from("X-Powered-By"), String::from("Waiter-rs"));
    }

    fn parse_html_file(view_path: &str) -> Result<(), std::io::Error> {
        fs::read_to_string(view_path)?
    }
}
