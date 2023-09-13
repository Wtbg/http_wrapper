use std::net::SocketAddr;

use axum::{
    extract::{ Path, State },
    http::StatusCode,
    response::{ Html, IntoResponse, Response },
    routing::{ get, post },
    Form,
    Router,
};
use faststr::FastStr;
use volo_gen::mini_redis::{
    MiniRedisServiceClient,
    MiniRedisServiceClientBuilder,
    SetRequest,
    Status,
    ValueResponse,
    GetRequest,
    DelRequest,
};
use volo_gen::*;
use volo_thrift::ResponseError;
use lazy_static::lazy_static;
use serde::Deserialize;
lazy_static! {
    static ref RPCCLIENT: MiniRedisServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        MiniRedisServiceClientBuilder::new("mini-redis").address(addr).build()
    };
}

#[derive(Deserialize, Debug)]
struct FormKeyValue {
    key: String,
    value: String,
}

#[derive(Deserialize, Debug)]
struct FormKey {
    key: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ping_http", get(ping_http))
        .route("/ping_rpc", get(ping_rpc))
        .route("/set_rpc", post(set_rpc))
        .route("/get_rpc", post(get_rpc))
        .route("/del_rpc", post(del_rpc));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn ping_http() -> (StatusCode, impl IntoResponse) {
    (StatusCode::OK, "pong from http")
}

async fn ping_rpc() -> (StatusCode, impl IntoResponse) {
    let resp = RPCCLIENT.ping().await;
    match resp {
        Ok(Status::Ok) => (StatusCode::OK, "pong from rpc"),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "error"),
    }
}

async fn set_rpc(Form(setkey): Form<FormKeyValue>) -> (StatusCode, impl IntoResponse) {
    let resp = RPCCLIENT.set(SetRequest {
        key: FastStr::from(setkey.key),
        value: FastStr::from(setkey.value),
    }).await;
    match resp {
        Ok(Status::Ok) => (StatusCode::OK, "set ok"),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "error"),
    }
}

async fn get_rpc(Form(getkey): Form<FormKey>) -> (StatusCode, impl IntoResponse) {
    let resp = RPCCLIENT.get(GetRequest {
        key: FastStr::from(getkey.key),
    }).await;
    match resp {
        Ok(resp) => {
            if let Some(value) = resp.value {
                (StatusCode::OK, value.as_str().to_string())
            } else {
                (StatusCode::OK, "(nil)".to_string())
            }
        },
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "error".to_string()),
    }
}

async fn del_rpc(Form(getkey): Form<FormKey>) -> (StatusCode, impl IntoResponse) {
    let resp = RPCCLIENT.del(DelRequest {
        key: FastStr::from(getkey.key),
    }).await;
    match resp {
        Ok(Status::Ok) => (StatusCode::OK, "del ok"),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "error"),
    }
}
