# Rust OpenAI Vision API Integration

This Rust project provides integration with OpenAI's Vision API for processing and analyzing images. It demonstrates how to use the GPT-4o-mini model to generate descriptions and translations of text in manga images.

## Features

- Image processing and base64 encoding
- OpenAI Vision API integration
- Manga text translation (Portuguese)
- Performance timing measurements
- Environment variable configuration

## Prerequisites

- Rust (latest stable version)
- OpenAI API key

## Setup

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd Rust-LLM
   ```

2. Create a `.env` file in the project root and add your OpenAI API key:
   ```
   OPENAI_API_KEY=your_api_key_here
   ```

3. Install dependencies:
   ```bash
   cargo build
   ```

## Usage

1. Place your image file in the project directory

2. Update the image path in `src/main.rs`:
   ```rust
   let image_path = "path/to/your/image.png";
   ```

3. Run the project:
   ```bash
   cargo run
   ```

## Dependencies

- `async-openai`: OpenAI API client
- `base64`: Image encoding
- `dotenv`: Environment variable management
- `tokio`: Async runtime
- `serde_json`: JSON serialization

## Error Handling

The application handles various error cases:
- Environment file loading failures
- Image file reading errors
- API communication issues

## Performance

The application includes performance monitoring, displaying response times in minutes, seconds, and milliseconds for API calls.

## License

This project is open source and available under the MIT License.