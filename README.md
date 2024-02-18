# Jemini

Jemini is a Rust library that provides a simple and intuitive interface for interacting with the Google Gemini REST API.

It exists because I couldn't help but notice that there's no Rust option in their docs... shame on you Google.

## Goals:

- Easy-to-use API client for Google Gemini services.
- Convenient methods for generating content using the Gemini API.

## Getting Started

To get started with Jemini, add it as a dependency to your Rust project's `Cargo.toml` file:

```toml
[dependencies]
jemini = "0.1.0"
```

Then, in your `main.rs`:

```rust
use jemini::{JeminiClient, GeminiError};

#[tokio::main]
async fn main() -> Result<(), GeminiError> {
    let client = JeminiClient::new()?;
    let response: serde_json::Value = client.text_only("What is the meaning of life?").await?;

    println!("{:#?}", response);

    Ok(())

}
```

> NOTE: `GeminiError` will `#transparent` most of / all of the errors possible from the dependency crates so be aware of that should you encounter problems.

## Usage

The `JeminiClient` provides a simple interface for making requests to the Gemini API. Here's an example of how to use it:

```rust
use jemini::JeminiClient;

#[tokio::main]
async fn main() {
    let client = JeminiClient::new().unwrap();
    let response = client.text_only("Write a story about a magic backpack.").await.unwrap();
    println!("{:#?}", response);
}
```

## Testing

To run the tests for Jemini, use the following command:

```sh
cargo test
```

## Contributing

Contributions to Jemini are welcome! Please feel free to open an issue or submit a pull request if you have improvements or bug fixes.

## License

Jemini is licensed under the MIT License.
