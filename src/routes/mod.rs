use crate::{
    request::HttpRequest,
    response::{HttpResponse, HttpResponseBuilder},
    routes::router::{AddRoute, Router},
};
use regex::Regex;

mod route;
mod router;

fn hello_world() -> HttpResponse {
    HttpResponseBuilder::new()
        .set_body("Hello, World!".into())
        .build()
}

fn hello_user(params: Vec<String>) -> HttpResponse {
    HttpResponseBuilder::new()
        .set_body(format!("Hello, {}", params[0]))
        .build()
}

pub fn handle_request(request: HttpRequest) -> HttpResponse {
    let router = Router::new()
        .add_route(
            Regex::new(r"^/$").unwrap(),
            hello_world as fn() -> HttpResponse,
        )
        .add_route(
            Regex::new(r"^/user/([^/]+)$").unwrap(),
            hello_user as fn(Vec<String>) -> HttpResponse,
        );

    router.route(request)
}
