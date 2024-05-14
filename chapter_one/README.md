#CHAPTER ONE- GETTING STARTED

Welcome to Chapter One of our Rust learning journey! In this chapter, we will cover the basics of getting started with Rust programming language, including:
-  Installation of rust
-  Writing your first program using `rustc`
-  Using Cargo to manage Rust projects.

## Installation

To setup rust in linux:
1. Open a terminal window.
2. Download and run the `rustup` installation script by using `curl`:
    ```bash
   $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
3. Follow the on-screen instructions to complete the installation. This will download and install the latest stable version of Rust along with Cargo.
4. After installation is complete, `rustup` will add Rust to your system PATH automatically. To start using Rust, you might need to restart your terminal.
5. Verify that Rust is installed by running:

    ```bash
    $ rustc --version
    ```

    This command should display the installed version of Rust.

## Hello, World!

To create the first program with rust:
1. Open terminal window
2. Create a directory as follows:
	```bash
	$ mkdir helloWorld
	```
3. Create the rust file and enter the code in the helloWorld/helloWorld.rs file as follows:
	```bash
	$ vi helloWorld.rs
	```
4. Compile the rust file & run it as follows:
	```bash
	$ rustc helloWorld.rs
	$ ./helloWorld
	```

## Hello Cargo

Cargo is Rust's package manager and system builder.
### Common Cargo comands

Some of the more common commands used are as follows:
- `cargo new <project-name>`: Create a new Rust project.
- `cargo build`: Compile the current Rust project and create a binary in **/target/debug**. It also creates a TOML (Tom Obvious, Minimal file) that contains package configurations such as dependencies.
- `cargo build --release`: It compiles the project with optimization when it is ready for release.It makes the code to run faster. It is stored in **/target/release**.
- `cargo run`: Compile and run the current Rust project.
- `cargo test`: Run tests for the current Rust project.
- `cargo check`: Build the project without producing binary to check for errors
- `cargo doc`: Generate documentation for the current Rust project in **/target/doc**
- `cargo clean`: Remove build artifacts and target directory.
###Test

To test the Rust project created with Cargo:
1. Open terminal window.
2. Navigate to the project directory by executing the following command:
	```bash
	$ cd hello_world
	```
3. Build & run the project to display 'Hello World' by executing the following command:
	```bash 	
	$ cargo run
	```	
