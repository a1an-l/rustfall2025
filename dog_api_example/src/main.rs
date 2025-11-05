use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum AppError {
    Api(String),
    Network(String),
    File(String),
    Json(String),
}

#[derive(Debug)]
enum ApiResult {
    Success(String), // returns the file path of the saved image
    Failure(AppError),
}

fn fetch_random_dog_image() -> Result<DogImage, AppError> {
    let url = "https://dog.ceo/api/breeds/image/random";

    let response = ureq::get(url).call().map_err(|e| AppError::Network(e.to_string()))?;
    if response.status() != 200 {
        return Err(AppError::Api(format!("HTTP error: {}", response.status())));
    }

    response
        .into_json::<DogImage>()
        .map_err(|e| AppError::Json(format!("Failed to parse JSON: {}", e)))
}

fn download_image(url: &str, file_name: &str) -> Result<(), AppError> {
    let mut response = ureq::get(url)
        .call()
        .map_err(|e| AppError::Network(format!("Failed to fetch image: {}", e)))?;

    let mut file = File::create(file_name)
        .map_err(|e| AppError::File(format!("Failed to create file: {}", e)))?;

    let mut buffer = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut buffer)
        .map_err(|e| AppError::File(format!("Failed to read image data: {}", e)))?;

    file.write_all(&buffer)
        .map_err(|e| AppError::File(format!("Failed to write image data: {}", e)))?;

    Ok(())
}

fn fetch_and_download_image(i: usize) -> ApiResult {
    match fetch_random_dog_image() {
        Ok(dog_image) => {
            let file_name = format!("dog_image_{}.jpg", i);
            match download_image(&dog_image.message, &file_name) {
                Ok(_) => ApiResult::Success(file_name),
                Err(e) => ApiResult::Failure(e),
            }
        }
        Err(e) => ApiResult::Failure(e),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Fetcher");
    println!("=====================\n");

    for i in 1..=5 {
        println!("Fetching and downloading random dog image #{}", i);

        match fetch_and_download_image(i) {
            ApiResult::Success(file_path) => {
                println!("✅ Success! Image saved to '{}'", file_path);
            }
            ApiResult::Failure(AppError::Api(e)) => println!("API Error: {}", e),
            ApiResult::Failure(AppError::Network(e)) => println!("Network Error: {}", e),
            ApiResult::Failure(AppError::File(e)) => println!("File Error: {}", e),
            ApiResult::Failure(AppError::Json(e)) => println!("JSON Error: {}", e),
        }

        println!();
    }

    Ok(())
}
