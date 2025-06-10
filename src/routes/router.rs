use crate::{
    database::connect,
    request::HttpRequest,
    response::{HttpResponse, HttpResponseBuilder},
    routes::route::{Handler, New, Route},
};
use regex::Regex;
use rusqlite::Connection;

pub struct Router {
    routes: Vec<Route>,
}
impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn route(&self, request: HttpRequest) -> HttpResponse {
        for route in &self.routes {
            if let Some(caps) = route.uri().captures(request.uri()) {
                let mut matches: Vec<String> = Vec::new();
                for i in 1..caps.len() {
                    matches.push(caps[i].into());
                }

                return match &route.handler() {
                    Handler::Base(f) => f(),
                    Handler::Req(f) => f(request),
                    Handler::Params(f) => f(matches),
                    Handler::Conn(f) => {
                        let conn = connect(
                            crate::database::DatabaseConnectionType::Sqlite,
                            dotenv::var("DATABASE_SQLITE").unwrap(),
                        );
                        return match conn {
                            Ok(c) => f(&c),
                            Err(e) => {
                                eprintln!("{}", e);
                                HttpResponseBuilder::r_500()
                            }
                        };
                    }
                    Handler::ReqParams(f) => f(request, matches),
                    Handler::ReqConn(f) => {
                        let conn = connect(
                            crate::database::DatabaseConnectionType::Sqlite,
                            dotenv::var("DATABASE_SQLITE").unwrap(),
                        );
                        return match conn {
                            Ok(c) => f(request, &c),
                            Err(e) => {
                                eprintln!("{}", e);
                                HttpResponseBuilder::r_500()
                            }
                        };
                    }
                    Handler::ParamsConn(f) => {
                        let conn = connect(
                            crate::database::DatabaseConnectionType::Sqlite,
                            dotenv::var("DATABASE_SQLITE").unwrap(),
                        );
                        return match conn {
                            Ok(c) => f(matches, &c),
                            Err(e) => {
                                eprintln!("{}", e);
                                HttpResponseBuilder::r_500()
                            }
                        };
                    }
                    Handler::ReqParamsConn(f) => {
                        let conn = connect(
                            crate::database::DatabaseConnectionType::Sqlite,
                            dotenv::var("DATABASE_SQLITE").unwrap(),
                        );
                        return match conn {
                            Ok(c) => f(request, matches, &c),
                            Err(e) => {
                                eprintln!("{}", e);
                                HttpResponseBuilder::r_500()
                            }
                        };
                    }
                };
            }
        }
        HttpResponseBuilder::r_404()
    }
}

pub trait AddRoute<T> {
    fn add_route(self, uri: Regex, handeler: T) -> Self;
}

macro_rules! impl_add_route {
    ($($fn_type:ty => $variant:ident),* $(,)?) => {
        $(
            impl AddRoute<$fn_type> for Router {
                fn add_route(mut self, uri: Regex, handler: $fn_type) -> Self {
                    self.routes.push(Route::new(uri, handler));
                    self
                }
            }
        )*
    };
}

impl_add_route! {
    fn() -> HttpResponse => Base,
    fn(HttpRequest) -> HttpResponse => Req,
    fn(Vec<String>) -> HttpResponse => Params,
    fn(&Connection) -> HttpResponse => Conn,
    fn(HttpRequest, Vec<String>) -> HttpResponse => ReqParams,
    fn(HttpRequest, &Connection) -> HttpResponse => ReqConn,
    fn(Vec<String>, &Connection) -> HttpResponse => ParamsConn,
    fn(HttpRequest, Vec<String>, &Connection) -> HttpResponse => ReqParamsConn,
}
