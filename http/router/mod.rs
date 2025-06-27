use crate::request::Request;
use crate::{database::DBConnection, http::method::HttpMethod, response::Response};
use paste::paste;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::future::Future;
use std::io::Read;
use std::pin::Pin;
use std::sync::Arc;

#[allow(clippy::type_complexity)]
pub enum Route {
    Base(Box<dyn Fn() -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync + 'static>),
    Params(
        Box<
            dyn Fn(HashMap<String, String>) -> Pin<Box<dyn Future<Output = Response> + Send>>
                + Send
                + Sync
                + 'static,
        >,
    ),
    Database(
        Box<
            dyn Fn(&mut Arc<DBConnection>) -> Pin<Box<dyn Future<Output = Response> + Send>>
                + Send
                + Sync
                + 'static,
        >,
    ),
    DatabaseParams(
        Box<
            dyn Fn(
                    &mut Arc<DBConnection>,
                    HashMap<String, String>,
                ) -> Pin<Box<dyn Future<Output = Response> + Send>>
                + Send
                + Sync
                + 'static,
        >,
    ),
    Request(
        Box<
            dyn Fn(Request) -> Pin<Box<dyn Future<Output = Response> + Send>>
                + Send
                + Sync
                + 'static,
        >,
    ),
    RequestParams(
        Box<
            dyn Fn(
                    Request,
                    HashMap<String, String>,
                ) -> Pin<Box<dyn Future<Output = Response> + Send>>
                + Send
                + Sync
                + 'static,
        >,
    ),
    RequestDatabase(
        Box<
            dyn Fn(
                    Request,
                    &mut Arc<DBConnection>,
                ) -> Pin<Box<dyn Future<Output = Response> + Send>>
                + Send
                + Sync
                + 'static,
        >,
    ),
    RequestDatabaseParams(
        Box<
            dyn Fn(
                    Request,
                    &mut Arc<DBConnection>,
                    HashMap<String, String>,
                ) -> Pin<Box<dyn Future<Output = Response> + Send>>
                + Send
                + Sync
                + 'static,
        >,
    ),
}

impl Route {
    async fn run(
        &self,
        request: Request,
        params: HashMap<String, String>,
        conn: &mut Arc<DBConnection>,
    ) -> Response {
        match self {
            Route::Base(handler) => handler().await,
            Route::Params(handler) => handler(params).await,
            Route::Database(handler) => handler(conn).await,
            Route::DatabaseParams(handler) => handler(conn, params).await,
            Route::Request(handler) => handler(request).await,
            Route::RequestParams(handler) => handler(request, params).await,
            Route::RequestDatabase(handler) => handler(request, conn).await,
            Route::RequestDatabaseParams(handler) => handler(request, conn, params).await,
        }
    }
}

impl fmt::Debug for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Route::Base(_) => write!(f, "Route::Base(<function>)"),
            Route::Params(_) => write!(f, "Route::Params(<function>)"),
            Route::Database(_) => write!(f, "Route::Database(<function>)"),
            Route::DatabaseParams(_) => write!(f, "Route::DatabaseParams(<function>)"),
            Route::Request(_) => write!(f, "Route::Request(<function>)"),
            Route::RequestParams(_) => write!(f, "Route::RequestParams(<function>)"),
            Route::RequestDatabase(_) => write!(f, "Route::ReuqestDatabase(<function>)"),
            Route::RequestDatabaseParams(_) => {
                write!(f, "Route::RequestDatabaseConnection(<function>)")
            }
        }
    }
}

#[derive(Debug)]
pub struct RouteTree {
    children: HashMap<String, RouteTree>,
    param_children: HashMap<String, RouteTree>,
    handler: Option<Route>,
}

impl RouteTree {
    fn find_match<'a>(
        &'a self,
        segments: &[&str],
        params: &mut HashMap<String, String>,
    ) -> Option<&'a Route> {
        if segments.is_empty() {
            return self.handler.as_ref();
        }

        let current_segment = segments[0];
        let remaining_segments = &segments[1..];

        if let Some(next_node) = self.children.get(current_segment) {
            let result = next_node.find_match(remaining_segments, params);
            if result.is_some() {
                return result;
            }
        }

        for (name, param_node) in &self.param_children {
            if let Some(handler) = param_node.find_match(remaining_segments, params) {
                params.insert(
                    name.strip_prefix(":").unwrap_or(name).to_owned(),
                    current_segment.to_string(),
                );
                return Some(handler);
            }
        }

        None
    }

    fn add(&mut self, uri: String, handler: Route) {
        // If index
        if &uri == "/" {
            self.handler = Some(handler);
            return;
        }

        //if not index /../../../
        let (uri_, new_) = uri
            .strip_prefix('/')
            .unwrap_or(&uri)
            .split_once('/')
            .unwrap_or((&uri, ""));
        //split to get .. and ../../..
        let (uri, new) = (uri_.to_string(), new_.to_string());
        let is_uri_param = uri.starts_with(':');
        if new.is_empty() {
            if let Some(node) = if is_uri_param {
                self.param_children.get_mut(&uri)
            } else {
                self.children.get_mut(&uri)
            } {
                node.handler = Some(handler);
            } else if is_uri_param {
                self.param_children.insert(
                    uri.clone(),
                    RouteTree {
                        handler: Some(handler),
                        param_children: HashMap::new(),
                        children: HashMap::new(),
                    },
                );
            } else {
                self.children.insert(
                    uri.clone(),
                    RouteTree {
                        handler: Some(handler),
                        param_children: HashMap::new(),
                        children: HashMap::new(),
                    },
                );
            };

            return;
        }

        if let Some(node) = self.children.get_mut(&uri) {
            node.add(new, handler);
        } else {
            self.children.insert(
                uri.clone(),
                RouteTree {
                    handler: None,
                    param_children: HashMap::new(),
                    children: HashMap::new(),
                },
            );
            self.children.get_mut(&uri).unwrap().add(new, handler);
        }
    }

    fn new() -> Self {
        RouteTree {
            handler: None,
            param_children: HashMap::new(),
            children: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Router {
    routes: HashMap<HttpMethod, RouteTree>,
    static_routes: HashMap<String, String>,
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

pub trait IntoRouteHandler<Args> {
    fn into_route(self) -> Route;
}

macro_rules! impl_handler {
    // Case 0: No arguments
    (() => $variant:ident) => {
        impl<F, Fut> IntoRouteHandler<()> for F
        where
            F: Fn() -> Fut + Send + Sync + 'static,
            Fut: Future<Output = Response> + Send + 'static,
        {
            fn into_route(self) -> Route {
                Route::$variant(Box::new(move || Box::pin(self())))
            }
        }
    };

    // Case 1: One argument
    ($arg1_ty:ty => $variant:ident) => {
        impl<F, Fut> IntoRouteHandler<($arg1_ty,)> for F
        where
            F: Fn($arg1_ty) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = Response> + Send + 'static,
        {
            fn into_route(self) -> Route {
                Route::$variant(Box::new(move |arg1: $arg1_ty| Box::pin(self(arg1))))
            }
        }
    };

    // Case 2: Two arguments
    ($arg1_ty:ty, $arg2_ty:ty => $variant:ident) => {
        impl<F, Fut> IntoRouteHandler<($arg1_ty, $arg2_ty)> for F
        where
            F: Fn($arg1_ty, $arg2_ty) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = Response> + Send + 'static,
        {
            fn into_route(self) -> Route {
                Route::$variant(Box::new(move |arg1: $arg1_ty, arg2: $arg2_ty| {
                    Box::pin(self(arg1, arg2))
                }))
            }
        }
    };

    // Case 3: Three arguments
    ($arg1_ty:ty, $arg2_ty:ty, $arg3_ty:ty => $variant:ident) => {
        impl<F, Fut> IntoRouteHandler<($arg1_ty, $arg2_ty, $arg3_ty)> for F
        where
            F: Fn($arg1_ty, $arg2_ty, $arg3_ty) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = Response> + Send + 'static,
        {
            fn into_route(self) -> Route {
                Route::$variant(Box::new(
                    move |arg1: $arg1_ty, arg2: $arg2_ty, arg3: $arg3_ty| {
                        Box::pin(self(arg1, arg2, arg3))
                    },
                ))
            }
        }
    };
}

// --- UPDATED MACRO CALLS to match the new patterns ---
impl_handler!(() => Base);
impl_handler!(HashMap<String, String> => Params);
impl_handler!(&mut Arc<DBConnection> => Database); // Updated
impl_handler!(&mut Arc<DBConnection>, HashMap<String, String> => DatabaseParams); // Updated
impl_handler!(Request => Request);
impl_handler!(Request, HashMap<String, String> => RequestParams);
impl_handler!(Request, &mut Arc<DBConnection> => RequestDatabase); // Updated
impl_handler!(Request, &mut Arc<DBConnection>, HashMap<String, String> => RequestDatabaseParams); // Updated

macro_rules! route_method_impl {
    ($name:ident, $variant:ident) => {
        paste! {
            pub fn [<r_$name>]<F, Args>(&mut self, uri: &str, handler: F) -> &mut Self
            where
                F: IntoRouteHandler<Args>,
            {
                self.routes
                    .get_mut(&crate::http::method::HttpMethod::$variant)
                    .unwrap()
                    .add(uri.to_string(), handler.into_route());
                self
            }

            pub fn $name<F, Args>(mut self, uri: &str, handler: F) -> Self
            where
                F: IntoRouteHandler<Args>,
            {
                self.routes
                    .get_mut(&crate::http::method::HttpMethod::$variant)
                    .unwrap()
                    .add(uri.to_string(), handler.into_route());
                self
            }
        }
    };
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::from([
                (HttpMethod::Get, RouteTree::new()),
                (HttpMethod::Post, RouteTree::new()),
                (HttpMethod::Patch, RouteTree::new()),
                (HttpMethod::Put, RouteTree::new()),
                (HttpMethod::Delete, RouteTree::new()),
                (HttpMethod::Options, RouteTree::new()),
            ]),
            static_routes: HashMap::new(),
        }
    }

    pub fn serve_dir(mut self, uri: &str, dir: &str) -> Self {
        self.static_routes.insert(
            if uri.starts_with("/") {
                uri.to_string()
            } else {
                format!("/{}", uri)
            },
            if dir.starts_with("/") {
                dir.to_string()
            } else {
                format!("/{}", dir)
            },
        );
        self
    }

    pub fn r_serve_dir(&mut self, uri: &str, dir: &str) -> &mut Self {
        self.static_routes.insert(uri.to_string(), dir.to_string());
        self
    }

    route_method_impl!(get, Get);
    route_method_impl!(post, Post);
    route_method_impl!(patch, Patch);
    route_method_impl!(put, Put);
    route_method_impl!(delete, Delete);
    route_method_impl!(options, Options);

    pub async fn handle(&self, request: Request) -> Response {
        let route_and_params = {
            let root = self.routes.get(&request.method).unwrap();
            let segments: Vec<&str> = request.uri.split('/').filter(|s| !s.is_empty()).collect();
            let mut params = HashMap::new();
            root.find_match(&segments, &mut params)
                .map(|handler| (handler, params))
        };
        if let Some((r, p)) = route_and_params {
            r.run(request, p, &mut Arc::new(DBConnection::connect()))
                .await
        } else {
            for i in &self.static_routes {
                if request.uri.starts_with(i.0) {
                    let filename = format!(
                        ".{}/{}",
                        i.1,
                        request.uri.chars().skip(i.0.len()).collect::<String>()
                    );

                    if let Ok(mut file) = File::open(&filename) {
                        let mut buf = Vec::new();
                        if file.read_to_end(&mut buf).is_err() {
                            return Response::new()
                                .status(crate::http::status::HttpStatusCode::InternalServerError)
                                .build();
                        }
                        let mime_type = mime_guess::from_path(filename)
                            .first_or_text_plain()
                            .to_string();
                        return Response::new()
                            .body_raw(buf)
                            .header("Content-Type", mime_type.as_str())
                            .build();
                    }
                }
            }

            Response::new()
                .status(crate::http::status::HttpStatusCode::NotFound)
                .build()
        }
    }
}
