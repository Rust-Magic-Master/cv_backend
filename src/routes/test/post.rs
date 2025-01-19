use axum::http::StatusCode;
use axum::Json;
use opencv::imgproc::COLOR_BGR2GRAY;
use opencv::prelude::*;
use opencv::{imgcodecs, imgproc::cvt_color_def};
use serde::{Deserialize, Serialize};
use std::env;
#[derive(Debug, Deserialize, Clone)]
pub struct PostRequest {
    pub file_name: String,
}
#[derive(Debug, Serialize)]
pub struct PostResoponse {
    pub processed_file_name: String,
}

pub fn get_image(body: Json<PostRequest>) -> Result<Mat, StatusCode> {
    let file_name = &body.0.file_name;
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let photos_folder = current_dir.join("assets");
    let image_path = photos_folder.join(file_name);
    let img = match imgcodecs::imread(image_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR) {
        Ok(img) => img,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    Ok(img)
}
pub fn pass_to_gray(img: Mat) -> Result<Mat, StatusCode> {
    let mut img_gray = Mat::default();
    match cvt_color_def(&img, &mut img_gray, COLOR_BGR2GRAY) {
        Ok(it) => it,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    Ok(img_gray)
}
#[axum::debug_handler]
pub async fn post_test(body: Json<PostRequest>) -> Result<Json<PostResoponse>, StatusCode> {
    let img = get_image(body.clone()).unwrap();
    println!("Image dimensions: {}x{}", img.cols(), img.rows());
    let img_gray = pass_to_gray(img).unwrap();
    imgcodecs::imwrite("assets/gray.jpg", &img_gray, &opencv::core::Vector::new()).unwrap();
    Ok(Json(PostResoponse {
        processed_file_name: String::from("gray.jpg"),
    }))
}
