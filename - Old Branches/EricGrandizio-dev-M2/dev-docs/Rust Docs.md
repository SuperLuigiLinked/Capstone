# Preface

**Before continuing, make sure you have installed all the necessary [Dependencies](dev-docs/Dependencies.md).**

In order to generate documentation or run tests, you will need to open the directory containing the Rust projects in the terminal.

This can be done in VS Code by looking to the titlebar, using the `Terminal` dropdown, and using the `New Terminal` command:

![image](https://user-images.githubusercontent.com/65352263/215339600-4126ca37-7715-4da5-acc3-e4817af63ef8.png)

This should open an interactive terminal within VS Code. The exact directory path will vary, but you should see something like this:

![image](https://user-images.githubusercontent.com/65352263/215339805-34c10a8e-364c-462c-817c-a6699bb9c1c7.png)

### Note

If you run any of the following commands in the root directory of this repository, it will compile both Wyn and RGE.

If you want to run these commands for only one of these libraries, navigate to the corresponding directory and run the commands there.

This will also compile any dependencies listed in the corresponding `Cargo.toml` file in said directory.

# Compiling

### cargo build

To compile Rust Code, run the command `cargo build` in the terminal.

This will, by default, generate **Debug** binaries.

To generate **Release** binaries, append the `--release` flag after the command (Ex: `cargo build --release`).

The `--release` flag will work for most of the following commands, as well.

### cargo clean

When compiling, the outputs and intermediate files will be placed in a new directory called `target`.

In order to do a clean rebuild, you must first run `cargo clean` to clear out the `target` directory.

### cargo check

If you want to merely check code for compilation warnings/errors, use the command `cargo check`.

### cargo clippy

A useful command similar to `cargo check`, but this enables more warnings/errors and is generally stricter.

# Documentation

### cargo doc

To generate Documentation, run the command `cargo doc` in the terminal.

This will compile the code and generate a local website with documentation derived from doc comments in the source code.

### --no-deps

By default, this will also generate the (potentially lengthy/time-consuming) documentation for all dependencies, as well.

To generate only the documentation for the current project, append the `--no-deps` flag.

### --document-private-items

By default, items in the code with private visibility do not have documentation generated.

To generate documentation for private items, append the `--document-private-items` flag.

### --open

The generated documentation files will be placed in the `target/doc` directory.

To open the documentation within your browser, append the `--open` flag.

# Testing

### cargo test

To run Tests, run the command `cargo test` in the terminal.

This will compile and run all the tests for the current project(s), then print the results out in the terminal.

User-Acceptance Tests will be ignored by default. In order to run a UA-Test, read the instructions from the corresponding User Story.

### VS Code - Test Explorer

If you have installed the Rust Extension Pack for VS Code (mentioned in [Dependencies](dev-docs/Dependencies.md)), you will see the Test Explorer below:

![image](https://user-images.githubusercontent.com/65352263/215343726-06cb6c31-2af8-4cd9-a018-7b964ce5dd31.png)

At the time of writing, the "Rust Test Explorer" extension only supports viewing/running Unit Tests.

Given that the tests for Wyn/RGE are generally not able to be run concurrently, they must be Integration Tests.

As such, you will likely not see anything in this menu, and must run tests from the terminal instead.
