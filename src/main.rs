#[macro_use]
extern crate lazy_static;
extern crate webpacker;
extern crate gotham;
extern crate hyper;
extern crate tera;

use gotham::handler::assets::FileOptions;
use webpacker::asset_path::AssetPath;
use gotham::state::State;
use gotham::router::builder::{
    build_simple_router,
    DefineSingleRoute,
    DrawRoutes
};
use gotham::router::Router;
use tera::{Context, Tera};
use webpacker::Manifest;
use std::ops::Deref;
use hyper::Method;
use std::env;

pub static ASSET_DIRECTORY: &'static str = "public";

lazy_static! {
    pub static ref TERA: Tera =
        Tera::new("app/views/**/*.tera").
            map_err(|e| {
                eprintln!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }).
            unwrap();

    pub static ref MANIFEST: Manifest =
        webpacker::manifest(None).unwrap();
}

pub fn asset_source(key: &str) -> String {
    AssetPath::new(ASSET_DIRECTORY, key, MANIFEST.deref()).into()
}

pub fn javascript_pack_tag(key: &str) -> String {
    let key = format!("{}.js", basename(key));
    asset_source(&key)
}

pub fn stylesheet_pack_tag(key: &str) -> String {
    let key = format!("{}.css", basename(key));
    asset_source(&key)
}

fn basename<'a>(s: &'a str) -> &'a str {
    s.rsplit('/').next().unwrap().
        split('.').next().unwrap()
}

pub fn index_page(state: State) -> (State, (mime::Mime, String)) {
    let mut context = Context::new();

    let sources = &[
        &asset_source("application.js"),
        &javascript_pack_tag("hello")
    ];

    let styles = &[
        "style/application.css",
        &stylesheet_pack_tag("hello")
    ];
    context.insert("application_styles", styles);
    context.insert("application_sources", sources);

    let rendered = TERA.render("landing_page/index.html.tera", &context).unwrap();
    (state, (mime::TEXT_HTML, rendered))
}

pub fn router() -> Router {
    build_simple_router(|route| {
        route.
            request(vec![Method::GET, Method::HEAD], "/").
            to(index_page);

        route.
            get("style/*").
            to_dir(
                FileOptions::new("app/assets/stylesheets").
                    with_cache_control("no-cache").
                    with_gzip(true).
                    build(),
            );

        route.
            get(&format!("{}/*", &ASSET_DIRECTORY)).
            to_dir(
                FileOptions::new("public").
                    with_cache_control("no-cache").
                    with_gzip(true).
                    build(),
            );
    })
}

pub fn main() {
    let port = env::var("PORT").expect("PORT env not found!");
    let addr = format!("0.0.0.0:{}", port);
    println!("Listening for requests at {}", addr);
    gotham::start(addr, router())
}
