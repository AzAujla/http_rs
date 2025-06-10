use crate::{
    request::HttpRequest,
    response::{HttpResponse, HttpResponseBuilder},
};
use regex::Regex;

struct Route {
    uri: Regex,
    handeler: fn(HttpRequest, Vec<String>) -> HttpResponse,
}
impl Route {
    fn new(uri: Regex, handeler: fn(HttpRequest, Vec<String>) -> HttpResponse) -> Self {
        Self { uri, handeler }
    }
}

pub struct Router {
    routes: Vec<Route>,
}
impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn route(&self, request: HttpRequest) -> HttpResponse {
        for route in &self.routes {
            if let Some(caps) = route.uri.captures(request.uri()) {
                let mut matches: Vec<String> = Vec::new();
                for i in 1..caps.len() {
                    matches.push(caps[i].into());
                }
                return (route.handeler)(request, matches);
            }
        }
        HttpResponseBuilder::r_404()
    }

    pub fn add_route(
        mut self,
        uri: Regex,
        handeler: fn(HttpRequest, Vec<String>) -> HttpResponse,
    ) -> Self {
        self.routes.push(Route::new(uri, handeler));
        self
    }
}

fn hello_world(_: HttpRequest, _: Vec<String>) -> HttpResponse {
    HttpResponseBuilder::new()
        .set_body("Hello, World!".into())
        .build()
}

fn hello_user(_: HttpRequest, params: Vec<String>) -> HttpResponse {
    HttpResponseBuilder::new()
        .set_body(format!("Hello, {}", params[0]))
        .build()
}

pub fn handle_request(request: HttpRequest) -> HttpResponse {
    let router = Router::new()
        .add_route(Regex::new(r"^/$").unwrap(), hello_world)
        .add_route(Regex::new(r"^/user/([^/]+)$").unwrap(), hello_user);

    router.route(request)
}
