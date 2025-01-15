use api::account::AccountApi;
use db::{get_pg_pool, init_pg_pool};
use salvo::{
    http::Method,
    hyper::header::{ACCEPT, AUTHORIZATION},
    prelude::*,
};
use salvo_cors::{Any, Cors};
use task::schedule_task;

mod api;
mod auth;
mod db;
mod error;
mod listennotes;
mod models;
mod task;

pub trait Routers {
    fn build() -> Vec<Router>;
}

#[handler]
async fn hello_world(res: &mut Response) {
    res.render("Hello, world!");
}

#[handler]
async fn all_pass(res: &mut Response) {
    res.status_code(StatusCode::OK);
}

#[tokio::main]
async fn main() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ));
        })
        .level(log::LevelFilter::Debug)
        .level_for("reqwest", log::LevelFilter::Warn)
        .level_for("sqlx", log::LevelFilter::Warn)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    dotenv::dotenv().ok();
    let _ = init_pg_pool().await;
    schedule_task(get_pg_pool().clone());

    let mock_info = std::env::var("MOCK_API").unwrap();
    if mock_info.to_lowercase() == "true" {
        log::warn!("currently using mock api data");
    }

    let cors_handler = Cors::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![AUTHORIZATION, ACCEPT])
        .into_handler();

    let router = Router::new()
        .hoop(cors_handler)
        .push(Router::with_path("/").get(hello_world))
        .append(&mut AccountApi::build())
        .push(Router::with_path("<*path>").options(all_pass));

    let server_addr = std::env::var("SERVER_ADD").unwrap();

    let acceptor = TcpListener::new(server_addr).bind().await;
    Server::new(acceptor).serve(router).await;
}
