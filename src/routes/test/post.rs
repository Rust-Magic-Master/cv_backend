use axum::http::StatusCode;
use axum::Json;
use opencv::imgcodecs;
use opencv::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Clone)]
pub struct PostRequest {
    pub file_path: String,
}
#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub image: String,
}
#[axum::debug_handler]
pub async fn post_test(body: Json<PostRequest>) -> Result<(), StatusCode> {
    // 加载图像
    let image_path = &body.0.file_path;
    println!("image path:{:?}", image_path);
    let img = imgcodecs::imread(image_path, imgcodecs::IMREAD_COLOR).unwrap();
    // 打印图像信息
    println!("Image dimensions: {}x{}", img.cols(), img.rows());
    Ok(())
}
