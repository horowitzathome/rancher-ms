use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use tracing::info;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Dog {
    name: String,
    age: u32,
}

pub async fn write_dog() -> impl IntoResponse {
    info!("write_dog");

    let file_path = "/dog/data/dog.txt";

    let file_res = OpenOptions::new().create(true).append(true).open(file_path);

    if file_res.is_err() {
        let err = file_res.unwrap_err();
        return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string());
    }

    let mut file = file_res.unwrap();

    // Write data to the file
    let data_to_write = "Hello, Millie!";
    let wd_res = file.write_all(data_to_write.as_bytes());

    if wd_res.is_err() {
        let err = wd_res.unwrap_err();
        return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string());
    }

    (StatusCode::OK, "All good writing dog data".to_owned())
}

pub async fn get_dog_root() -> impl IntoResponse {
    info!("get_dog_root");
    _get_dog("Mille_Root ")
}

pub async fn get_dog() -> impl IntoResponse {
    info!("get_dog_hello");
    _get_dog("Mille_Hello ")
}

fn _get_dog(name: &str) -> impl IntoResponse {
    let now = Utc::now();
    let name = name.to_owned() + &now.to_rfc3339();

    info!("dog called, will return name {}", name);

    let dog = Dog { name: name, age: 1 };

    (StatusCode::OK, Json(dog))
}

pub async fn health() -> impl IntoResponse {
    info!("healthy_hello");
    Json("healthy_hello")
}

pub async fn health_root() -> impl IntoResponse {
    info!("healthy_root");
    Json("healthy_root")
}

pub async fn root_root() -> impl IntoResponse {
    info!("root_root");
    Json("root_root")
}

pub async fn root_hello() -> impl IntoResponse {
    info!("root_hello");
    Json("root_hello")
}
