#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::core::models;
use axum::routing::post;
use axum::{Json, Router};
use core::eq_solver::Task;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

pub mod core;
pub mod tests;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let app = Router::new()
        .route("/", post(handler))
        .layer(CorsLayer::permissive());

    axum::serve(listener, app).await.unwrap();
}

async fn match_op(cat: usize) -> Task {
    match cat {
        0 => Task::Eq(|x| x * x * x - x + 4.0),
        1 => Task::Eq(|x| x.sin()),
        2 => Task::Eq(|x| 12.0 / 11.0 * x - 1.0 / 11.0 * x * x * x - 4.0 / 11.0),
        3 => Task::Sys((
            |x, y| 0.3 - 0.1 * x * x - 0.2 * y * y,
            |x, y| 0.7 - 0.2 * x * x - 0.1 * x * y,
        )),
        4 => Task::Sys((|x, y| (y + 2.0).sin() - 1.5, |x, y| 0.5 - (x - 2.0).cos())),
        _ => unreachable!(),
    }
}

async fn match_method(method: usize) -> core::eq_solver::Method {
    match method {
        0 => core::eq_solver::Method::Chords,
        1 => core::eq_solver::Method::Secants,
        2 => core::eq_solver::Method::SimpleIt,
        _ => unreachable!(),
    }
}

async fn handler(Json(buf): Json<models::Request>) -> Json<models::Response> {
    let request = buf;

    let solver = core::eq_solver::Solver::new(
        match_op(request.category).await,
        match_method(request.method).await,
        100,
        request.eps,
        request.lb,
        request.rb,
    );

    let response = models::Response {
        x: solver.solve(),
        acc: solver.acc.take(),
        iters: solver.iters.take(),
        errors: solver.errors.take(),
    };

    println!("{:?}", response);
    Json(response)
}
