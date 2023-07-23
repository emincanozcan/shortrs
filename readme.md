# ShortRS - URL Shortener

ShortRS is a simple URL shortening application built with Rust. Users can utilize this application to shorten long URLs and share them with a shorter key. They can also use the shortened URLs to redirect to the original URLs.

## Technologies Used

- [Rust](https://www.rust-lang.org/): Programming language used for backend development.
- [Actix-Web](https://actix.rs/): Web framework for building the server.
- [Askama](https://docs.rs/askama/): Template engine for rendering HTML templates.
- [serde](https://serde.rs/): Serialization and deserialization library for handling JSON data.

## Usage

1. Clone the repository: `git clone https://github.com/your-username/url-shortener.git`
2. Change into the project directory: `cd url-shortener`
3. Build the project: `cargo build`
4. Run the application: `cargo run`
5. Access the application at `http://localhost:8080`

## Features

- Shorten long URLs into shorter keys.
- Redirect shortened URLs to the original URLs.
- View a list of all shortened URLs.

## Todo

- Implement authentication for user accounts.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, feel free to open an issue or create a pull request.

## License

This project is licensed under the [MIT License](LICENSE).

Happy shortening! 🚀
