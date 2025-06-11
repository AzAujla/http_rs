use std::{
    fs::{self, File},
    io::Read,
};

use crate::{
    database::connect,
    request::HttpRequest,
    response::{HttpResponse, HttpResponseBuilder},
    routes::route::{Handler, New, Route},
};
use mime_guess::from_path;
use regex::Regex;
use rusqlite::Connection;

pub struct Router {
    routes: Vec<Route>,
    static_file_dirs: Vec<(Option<String>, String)>,
}
impl Router {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            static_file_dirs: Vec::new(),
        }
    }

    /**
     * Handles a HTTP Request, returns HTTP Response
     * @return crate::response::HttpResponse
     */
    pub fn route(&self, request: HttpRequest) -> HttpResponse {
        // Routing for handlers
        for route in &self.routes {
            if let Some(caps) = route.uri().captures(request.uri()) {
                let mut matches: Vec<String> = Vec::new();
                for i in 1..caps.len() {
                    matches.push(caps[i].into());
                }

                return match &route.handler() {
                    Handler::Response(res) => res.clone().to_owned(),
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

        // File routing
        for dir in &self.static_file_dirs {
            match &dir.0 {
                Some(d) => {
                    if Regex::new(format!("^/{}/.+", d).as_str())
                        .unwrap()
                        .is_match(request.uri())
                    {
                        return Self::serve_file(
                            dir.1.clone(),
                            request.uri().chars().skip(d.len() + 1).collect::<String>(),
                        );
                    } else {
                        continue;
                    }
                }
                None => {
                    return Self::serve_file(dir.1.clone(), String::from(request.uri()));
                }
            }
        }

        HttpResponseBuilder::r_404()
    }

    /**
     * Add a folder that will serve static files.
     * @param dir: String - The path from the root folder of the directory to be served
     * @param uri: Option<String> - The uri path will start with this.
     */
    pub fn add_static_files_dir(mut self, dir: String, uri: Option<String>) -> Self {
        self.static_file_dirs.push((uri, dir));
        self
    }

    pub fn serve_file(dir: String, path: String) -> HttpResponse {
        let contents = fs::read(format!("{}/{}", dir, path));
        let mime = from_path(path).first_or_octet_stream();
        match contents {
            Ok(f) => HttpResponseBuilder::new()
                .set_body(String::from_utf8_lossy(&f).into_owned())
                .set_header("Content-Type".into(), mime.essence_str().into())
                .build(),
            Err(e) => {
                eprintln!("{}", e);
                HttpResponseBuilder::r_500()
            }
        }
    }
}

pub trait AddRoute<T> {
    fn add_route(self, uri: &str, handeler: T) -> Self;
}

macro_rules! impl_add_route {
    ($($fn_type:ty => $variant:ident),* $(,)?) => {
        $(
            impl AddRoute<$fn_type> for Router {
                fn add_route(mut self, uri: &str, handler: $fn_type) -> Self {
                    self.routes.push(Route::new(Regex::new(uri).unwrap(), handler));
                    self
                }
            }
        )*
    };
}

impl_add_route! {
    HttpResponse => Response,
    fn() -> HttpResponse => Base,
    fn(HttpRequest) -> HttpResponse => Req,
    fn(Vec<String>) -> HttpResponse => Params,
    fn(&Connection) -> HttpResponse => Conn,
    fn(HttpRequest, Vec<String>) -> HttpResponse => ReqParams,
    fn(HttpRequest, &Connection) -> HttpResponse => ReqConn,
    fn(Vec<String>, &Connection) -> HttpResponse => ParamsConn,
    fn(HttpRequest, Vec<String>, &Connection) -> HttpResponse => ReqParamsConn,
}
