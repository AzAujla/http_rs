use crate::{request::HttpRequest, response::HttpResponse};
use regex::Regex;
use rusqlite::Connection;

pub enum Handler {
    Base(fn() -> HttpResponse),
    Req(fn(HttpRequest) -> HttpResponse),
    Params(fn(Vec<String>) -> HttpResponse),
    Conn(fn(&Connection) -> HttpResponse),
    ReqParams(fn(HttpRequest, Vec<String>) -> HttpResponse),
    ReqConn(fn(HttpRequest, &Connection) -> HttpResponse),
    ParamsConn(fn(Vec<String>, &Connection) -> HttpResponse),
    ReqParamsConn(fn(HttpRequest, Vec<String>, &Connection) -> HttpResponse),
}
pub struct Route {
    uri: Regex,
    handler: Handler,
}

impl Route {
    pub fn handler(&self) -> &Handler {
        &self.handler
    }

    pub fn uri(&self) -> &Regex {
        &self.uri
    }
}

pub trait New<T> {
    fn new(uri: Regex, handeler: T) -> Self;
}

macro_rules! impl_route_new {
    ($($fn_type:ty => $variant:ident),* $(,)?) => {
        $(
            impl New<$fn_type> for Route {
                fn new(uri: Regex, handler: $fn_type) -> Self {
                    Route {
                        uri,
                        handler: Handler::$variant(handler),
                    }
                }
            }
        )*
    };
}

impl_route_new! {
    fn() -> HttpResponse => Base,
    fn(HttpRequest) -> HttpResponse => Req,
    fn(Vec<String>) -> HttpResponse => Params,
    fn(&Connection) -> HttpResponse => Conn,
    fn(HttpRequest, Vec<String>) -> HttpResponse => ReqParams,
    fn(HttpRequest, &Connection) -> HttpResponse => ReqConn,
    fn(Vec<String>, &Connection) -> HttpResponse => ParamsConn,
    fn(HttpRequest, Vec<String>, &Connection) -> HttpResponse => ReqParamsConn,
}
