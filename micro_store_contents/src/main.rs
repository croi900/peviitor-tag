mod api;
mod caching;
mod fetching;

use api::{get_source, update};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use tokio; // 1.26.0, features = ["macros"]

use caching::update_jobs_html;
use fetching::Job;

use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use std::net::SocketAddr;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// How many jobs to update
    #[arg(short, long, default_value_t = 1000000000)]
    size: u64,

    /// From which index in the DB to begin from
    #[arg(short, long, default_value_t = 0)]
    beg: u64,

    /// SHA256 of the url to output the page soure of to stdout
    #[arg()]
    id: Option<String>,

    /// Should update cache
    #[arg(short, long, default_value_t = false)]
    update: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.id {
        Some(id) => {
            let page_source = std::fs::read_to_string(format!("job_sources/{}.txt", id))
                .expect("[readerr?badid]");

            print!("{}", page_source);
        }
        None => {
            if args.update {
                //let jobs = fetching::get_jobs(args.beg, args.size).await;
                //caching::cache_jobs(jobs).await.unwrap();
                //
                fetching::download_jobs(args.beg, args.size).await;
            } else {
                // build our application with a route
                let app = Router::new()
                    .route("/source", get(get_source))
                    .route("/update", get(update));

                let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

                axum::Server::bind(&addr)
                    .serve(app.into_make_service()) //hello from vim
                    .await
                    .unwrap();
            }
        }
    }
}
