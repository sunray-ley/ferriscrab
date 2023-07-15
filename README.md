# Ferriscrab

Ferriscrab is a lightweight and high-performance web server written in Rust. It provides a simple, fast, and reliable way to build and deploy web applications.

## Features

- **High Performance**: Ferriscrab leverages the powerful features and async programming model of Rust to achieve exceptional performance

- **Lightweight**: The server has a minimal codebase without unnecessary complexity, making it easy to understand and extend

- **Flexibility**: Ferriscrab provides a set of configurable options to adapt to various web application requirements

- **Security**: With Rust's memory safety guarantees and rigorous code review, Ferriscrab offers reliable security

## Installation

Ensure that you have the Rust toolchain installed. Run the following commands in the terminal to build and install Ferriscrab:

```shell
cargo install ferriscrab
```

## Usage

### Starting

You can start the Ferriscrab server with the following command:

```shell
ferriscrab start
```

By default, Ferriscrab will run on localhost port 8000. You can access your application by visiting `http://localhost:8000` in a web browser.

### Configuration

Ferriscrab uses a configuration file named `config.toml` to manage server settings. You can modify it according to your needs. Here's an example configuration file:

```toml
[server]
host = "127.0.0.1"
port = 8000

[logging]
level = "info"
```

## Contributing

Contributions to Ferriscrab are welcome! If you have any suggestions or questions, please open an issue or pull request on GitHub.

## License

Ferriscrab is distributed under the MIT License. Please refer to the [LICENSE](https://github.com/sunray-ley/ferriscrab/blob/main/LICENSE) file for more information.
