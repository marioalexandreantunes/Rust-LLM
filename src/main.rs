mod images;
mod ocr;

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use dotenv::dotenv;
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    println!("Executando OCR com arquivo...");
    // Get API key from environment variable or use a default one
    let api_key = env::var("OCR_API_KEY").unwrap_or_else(|_| "K81724976588957".to_string());
    // Use a local image file instead of a URL
    let image_path = "/Users/marioantunes/Desktop/Rust-LLM/ocr.png".to_string();

    // Create an ImageSource from the file path
    let image_source = ocr::ImageSource::FilePath(image_path);

    let result = ocr::ocr(api_key.clone(), image_source).await?;

    println!("{}", result);

    // Example using Base64 variant
    println!("\nExecutando OCR com base64...");
    // Read the image file and convert to base64
    let image_path = "/Users/marioantunes/Desktop/Rust-LLM/ocr.png";
    let image_data = fs::read(image_path)?;
    let base64_image = BASE64.encode(image_data);

    // Create an ImageSource from the base64 string
    let image_source_base64 = ocr::ImageSource::Base64(base64_image);

    // Call OCR with base64 image
    let result_base64 = ocr::ocr(api_key, image_source_base64).await?;

    println!("result_base64 {}", result_base64);

    println!("\nProcessando imagem com GPT-4 Vision...");
    // Use the manga.png file for image processing
    let manga_image_path = "/Users/marioantunes/Desktop/Rust-LLM/manga.png";
    images::images(manga_image_path).await?;
    Ok(())
}
