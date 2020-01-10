use std::collections::HashMap;
use std::io::prelude::*;
use std::fs;
use chrono::{DateTime, Utc};
use common_types;

pub struct HttpRequest {
    path: String,
    http_version: HttpVersion,
    verb: HttpVerbs,
    headers: HashMap<String, String>,
    content,
}

impl HttpRequest {
    
}

