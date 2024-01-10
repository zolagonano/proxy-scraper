# Proxy Scraper

The **Proxy Scraper** is a Rust command-line tool that allows users to scrape proxy information from URLs.

## Features

- **Scraping:** Fetch and scrape proxy information from a specified URL.
- **Proxy Types:** Currently supports MTProxy with extensibility for additional proxy types.
- **Asynchronous:** Utilizes asynchronous programming using the Tokio runtime for improved performance.

## Usage

### Installation

1. Ensure you have Rust and Cargo installed. If not, follow the instructions at [Rust Installation](https://www.rust-lang.org/tools/install).

2. Clone the repository:

    ```bash
    git clone https://github.com/zolagonano/proxy-scraper.git
    ```

3. Navigate to the project directory:

    ```bash
    cd proxy-scraper
    ```

4. Build the project:

    ```bash
    cargo build --release
    ```

### Command-line Usage

Run the built executable with the desired parameters:

```bash
./target/release/proxy-scraper --source <PROXY_SOURCE_URL> --proxy_type <PROXY_TYPE>
```

- `<PROXY_SOURCE_URL>`: The URL containing proxy information.
- `<PROXY_TYPE>`: The type of proxy to scrape (default: "mtproxy").

### Example

```bash
./target/release/proxy-scraper --source https://example.com/proxies --proxy_type mtproxy
```

## Configuration

The tool uses [argh](https://crates.io/crates/argh) for command-line argument parsing. The available options are:

- `--source`: Specifies the URL source for proxy information.
- `--proxy_type`: Specifies the type of proxy to scrape (default: "mtproxy").

## Dependencies

- [reqwest](https://crates.io/crates/reqwest): HTTP client for making requests.
- [tokio](https://crates.io/crates/tokio): Asynchronous runtime for Rust.
- [argh](https://crates.io/crates/argh): A simple argument parsing library.

## Building from Source

To build the project from source, follow these steps:

1. Clone the repository:

    ```bash
    git clone https://github.com/your_username/proxy-scraper.git
    ```

2. Navigate to the project directory:

    ```bash
    cd proxy-scraper
    ```

3. Build the project:

    ```bash
    cargo build --release
    ```

## Support

If you find My works helpful and would like to support me, consider making a donation. Your contributions will help to ensure the ongoing maintenance and improvement of these projects.

[https://zolagonano.github.io/support](https://zolagonano.github.io/support)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
