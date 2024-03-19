#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::core::models;
use axum::routing::post;
use axum::{Json, Router};
use core::eq_solver::Method;
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

async fn match_op(cat: usize) -> (fn(f32) -> f32, Vec<f32>) {
    match cat {
        0 => (|x| x.powi(3) / 3.0, vec![]),
        1 => (|x| -x.cos(), vec![]),
        2 => (|x| x.exp(), vec![]),
        3 => (|x| x.ln(), vec![0.0]),
        4 => (|x| 0.5 * ((1.0 - x).ln() - (x + 1.0).ln()), vec![-1.0, 1.0]),
        _ => unreachable!(),
    }
}

async fn match_method(method: usize) -> core::eq_solver::Method {
    match method {
        0 => Method::LeftRect,
        1 => Method::RightRect,
        2 => Method::MidRect,
        3 => Method::Trap,
        4 => Method::Simpson,
        _ => unreachable!(),
    }
}

async fn handler(Json(buf): Json<models::Request>) -> Json<models::Response> {
    let request = buf;
    println!("{:?}", request);
    let (task, mut bounds) = match_op(request.category).await;
    bounds.push(request.lb);
    bounds.push(request.rb);
    bounds.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let solver = core::eq_solver::Solver::New(
        task,
        match_method(request.method).await,
        request.N,
        request.eps,
        request.lb,
        request.rb,
        bounds,
    );

    let mut last = 0.0;
    let mut res = solver.solve();

    while (res - last).abs() >= request.eps {
        solver.n.set(solver.n.get() * 2);
        last = res;
        res = solver.solve();
    }

    let response = models::Response {
        x: res,
        N: solver.n.get(),
        acc: (last - res).abs(),
        errors: solver.errors.take(),
    };

    println!("{:?}", response);
    Json(response)
}
