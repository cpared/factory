# factory

## Initialize new Rust project

Cargo is the Rust package manager and build tool. To initialize a new Hello World project on Rust we just need to write the following command on a terminal.

> Cargo new <PROJECT_NAME>

This will automatically create a a new folder with all our Rust code inside of it and with a Cargo.toml file too. This last file has all the program configuration such as repository name, version or dependencies.

> foo
 --- Cargo.toml
 --- src
      |--- main.rs

## Build your code

When you want to compile your code you be first need to build it, and the command line for that is the following one.

> Cargo build

This will throw you all the error and warning logs. If its all right, you can procede with the next command line

> Cargo run

This command will execute your code.
