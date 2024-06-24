use std::collections::HashMap;

use log::warn;
use proxy_wasm::traits::{Context, HttpContext};
use proxy_wasm::types::Action;

const CONTENT_LENGTH: &str = "Content-Length";
const CONTENT_TYPE: &str = "Content-Type";
const RESPONSE_CODE: &str = "x-code";

pub struct PongHttpContext {
    response_headers: HashMap<String, String>,
    response_code: u32,
}

impl Default for PongHttpContext {
    fn default() -> Self {
        PongHttpContext {
            response_code: 200,
            response_headers: HashMap::new(),
        }
    }
}

impl Context for PongHttpContext {}

impl PongHttpContext {
    fn prepare_response_headers(&mut self) {
        let headers = self.get_http_request_headers();
        let mut response_headers = headers.into_iter().filter(|(k, _v)| {
            k.len() > 2 && k.starts_with("r-")
        }).map(|(k, v)| {
            (k.get(2..).unwrap().to_string(), v)
        }).collect::<HashMap<String, String>>();

        if let Some(query_bytes) = self.get_property(vec!["request", "query"]) {
            let query_term: Vec<String> = String::from_utf8(query_bytes).unwrap().split('&').map(|s| s.to_string()).collect();
            warn!("query_term is {:?}",query_term);
            query_term.iter().for_each(|x| {
                let k_v: Vec<&str> = x.split('=').collect();
                if let [key, value] = k_v.as_slice() {
                    response_headers.insert(key.to_string(), value.to_string());
                }
            });
        }

        if let Some(content_type) = self.get_http_request_header(CONTENT_TYPE) {
            warn!("content_type is {}",content_type);
            response_headers.insert(CONTENT_TYPE.to_string(), content_type);
        }
        self.response_headers = response_headers;
    }

    fn prepare_response_code(&mut self) {
        if let Some(response_code) = self.get_http_request_header(RESPONSE_CODE) {
            warn!("response code is {}",response_code);
            let response_code = response_code.parse().unwrap_or_default();
            if response_code > 0 {
                self.response_code = response_code;
            }
        }
    }

    fn response(&self, body_size: usize) {
        let mut body = None;
        let headers = self.response_headers.iter().map(|(k, v)| {
            (k.as_str(), v.as_str())
        }).collect();
        if body_size > 0 {
            if let Some(body_bytes) = self.get_http_request_body(0, body_size) {
                body = Some(body_bytes);
            }
        }
        self.send_http_response(self.response_code, headers, body.as_deref());
    }
}

impl HttpContext for PongHttpContext {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        self.prepare_response_headers();
        self.prepare_response_code();
        if let Some(content_length) = self.get_http_request_header(CONTENT_LENGTH) {
            if content_length.parse::<u32>().unwrap_or_default() > 0 {
                return Action::Continue;
            }
        }
        self.response(0);
        Action::Continue
    }

    fn on_http_request_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        if !end_of_stream {
            return Action::Pause;
        }
        if end_of_stream {
            self.response(body_size);
        }
        Action::Continue
    }
}