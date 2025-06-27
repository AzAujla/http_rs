use std::collections::HashMap;

use server::response::Response;

#[tokio::main]
async fn main() {
    server::App::new(
        "0.0.0.0",
        8000,
        server::router::Router::new()
            .get("/", async || {
                Response::new().body("Hello, World!\n").build()
            })
            .get("hello/:user", async |params: HashMap<String, String>| {
                Response::new()
                    .body(format!(
                        "Hello, {}!",
                        params.get("user").unwrap_or(&"Test User".into())
                    ))
                    .build()
            })
            .get("hello", async || {
                Response::new().body("Hello, World!\n").build()
            })
            .serve_dir("/", "/assets"),
    )
    .listen(true)
    .await
    .unwrap();
}
