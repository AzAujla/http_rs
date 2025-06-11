use crate::{
    request::HttpRequest,
    response::{HttpResponse, HttpResponseBuilder},
    routes::router::{AddRoute, Router},
};

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
            "^/user/([^/]+)$",
            hello_user as fn(Vec<String>) -> HttpResponse,
        )
        .add_route(
            "^/menu$",
            Router::serve_file(String::from("public"), String::from("menu.html")),
        )
        .add_route(
            "^/$",
            Router::serve_file(String::from("public"), String::from("index.html")),
        );

    router.route(request)
}
