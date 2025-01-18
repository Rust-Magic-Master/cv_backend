use opencv::prelude::*;
use opencv::imgcodecs;
use opencv::highgui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载图像
    let image_path = "test.jpg";
    let img = match imgcodecs::imread(image_path, imgcodecs::IMREAD_COLOR) {
        Ok(img) => {
            if img.empty() {
                return Err("Failed to load image - image is empty".into());
            }
            img
        },
        Err(e) => return Err(format!("Failed to load image: {:?}", e).into())
    };

    // 打印图像信息
    println!("Image dimensions: {}x{}", img.cols(), img.rows());
    
    // 显示图像
    if let Err(e) = highgui::imshow("window", &img) {
        return Err(format!("Failed to display image: {:?}", e).into());
    }
    
    highgui::wait_key(0)?;
    Ok(())
}
