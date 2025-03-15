use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::fmt;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct OcrResponse {
    #[serde(rename = "ParsedResults")]
    parsed_results: Vec<ParsedResult>,
    #[serde(rename = "IsErroredOnProcessing")]
    is_errored: Option<bool>,
    #[serde(rename = "ErrorMessage")]
    error_message: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct ParsedResult {
    #[serde(rename = "ParsedText")]
    parsed_text: String,
}

#[derive(Debug)]
pub struct OcrResult {
    pub text: String,
}

impl fmt::Display for OcrResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[derive(Debug)]
pub enum ImageSource {
    FilePath(String),
    Base64(String),
}

pub async fn ocr(
    api_key: String,
    image_source: ImageSource,
) -> Result<OcrResult, Box<dyn std::error::Error>> {
    let client = Client::new();

    // Create a multipart form based on the image source
    let mut form = reqwest::multipart::Form::new().text("language", "por");

    let start = std::time::Instant::now();
    match image_source {
        ImageSource::FilePath(path) => {
            // Read the local image file
            let image_data = std::fs::read(&path)?;
            let file_name = Path::new(&path)
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or_else(|| "image.png".to_string());

            form = form.part(
                "file",
                reqwest::multipart::Part::bytes(image_data).file_name(file_name),
            );
        }
        ImageSource::Base64(base64_str) => {
            // Format the base64 string with the required data URI prefix
            let formatted_base64 = format!("data:image/jpeg;base64,{}", base64_str);
            form = form.text("base64Image", formatted_base64);
        }
    };

    let res = client
        .post("https://api.ocr.space/parse/image")
        .header("apikey", api_key)
        .multipart(form)
        .send()
        .await?;

    // Check status code before parsing response
    if res.status() != StatusCode::OK {
        return Err(format!("API request failed with status: {}", res.status()).into());
    }

    let response: OcrResponse = res.json().await?;

    // Check if the API reported an error
    if let Some(true) = response.is_errored {
        let error_msg = response
            .error_message
            .unwrap_or_else(|| vec!["Unknown OCR processing error".to_string()])
            .join(", ");
        return Err(error_msg.into());
    }

    // Collect all extracted text into a single string
    let mut full_text = String::new();
    for result in response.parsed_results {
        println!("Extracted text: {}", result.parsed_text);
        if !full_text.is_empty() {
            full_text.push('\n');
        }
        full_text.push_str(&result.parsed_text);
    }
    // Calcula e exibe o tempo de resposta
    let duration = start.elapsed();
    let minutes = duration.as_secs() / 60;
    let seconds = duration.as_secs() % 60;
    let milliseconds = duration.subsec_millis();
    // milisegundos so com 3 digitos
    println!(
        "Tempo de resposta: {}'m, {}'s e {:03}'ms",
        minutes, seconds, milliseconds
    );

    Ok(OcrResult { text: full_text })
}
