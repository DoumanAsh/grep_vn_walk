extern crate actix;
extern crate actix_web;
extern crate futures;

use std::time;

use self::actix_web::http;

pub use self::futures::Future;
pub use self::actix::System;
pub use self::actix_web::HttpMessage;
pub use self::actix_web::client::{ClientRequest, ClientRequestBuilder, SendRequest};

/// Number of seconds to wait for connection
const CONN_TIMEOUT_S: u64 = 5;

pub trait ClientRequestExt {
    fn send_with_timeout(self, timeout: u64) -> SendRequest;
}
impl ClientRequestExt for ClientRequest {
    fn send_with_timeout(self, timeout: u64) -> SendRequest {
        self.send()
            .timeout(time::Duration::new(timeout, 0))
            .conn_timeout(time::Duration::new(CONN_TIMEOUT_S, 0))
            .wait_timeout(time::Duration::new(timeout, 0))
    }
}

pub trait ClientRequestBuilderExt {
    fn set_default_headers(self) -> Self;
}

impl ClientRequestBuilderExt for ClientRequestBuilder {
    fn set_default_headers(mut self) -> Self {
        self.header(
            http::header::USER_AGENT,
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/49.0.2623.87 Safari/537.36",
        ).header(http::header::ACCEPT_ENCODING, "gzip, deflate");
        self
    }
}

pub fn get(url: &str) -> ClientRequestBuilder {
    ClientRequest::get(url).set_default_headers()
}
