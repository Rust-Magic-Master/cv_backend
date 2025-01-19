use axum::http::StatusCode;
use axum::Json;
use opencv::imgproc::COLOR_BGR2GRAY;
use opencv::prelude::*;
use opencv::{imgcodecs, imgproc::cvt_color_def};
use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct PostRequest {
    pub root_path: String,
    pub file_name: String,
}
pub fn get_image(body: Json<PostRequest>) -> Result<Mat, StatusCode> {
    let root_path = &body.0.root_path;
    let file_name = &body.0.file_name;
    let image_path = format!("{}/{}", root_path, file_name);
    println!("file path:{:?}", root_path);
    println!("file name:{:?}", file_name);
    let img = match imgcodecs::imread(&image_path, imgcodecs::IMREAD_COLOR) {
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
pub async fn post_test(body: Json<PostRequest>) -> Result<(), StatusCode> {
    let img = get_image(body.clone()).unwrap();
    println!("Image dimensions: {}x{}", img.cols(), img.rows());
    let img_gray = pass_to_gray(img).unwrap();
    let img_gray_path = format!("{}/gray.jpg", body.0.root_path);
    imgcodecs::imwrite(&img_gray_path, &img_gray, &opencv::core::Vector::new()).unwrap();
    Ok(())
}
