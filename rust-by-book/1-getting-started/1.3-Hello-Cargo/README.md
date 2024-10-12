## Introduction to Rust Cargo

### 1. Create a project with Cargo
```bash
$ cargo new hello_cargo
```
It has also initialized a new Git repository along with a .gitignore file. Git files won’t be generated if you run cargo new within an existing Git repository; you can override this behavior by using cargo new --vcs=git.

Note: Cargo.toml is a configuration file for Rust projects. It contains metadata about the project and dependencies of the project.

### 2. Change to the project directory and run the following command to build the project.
```bash
$ cargo build
```
This command creates an executable file in target/debug/hello_cargo (or target\debug\hello_cargo.exe on Windows) rather than in your current directory. Because the default build is a debug build, Cargo puts the binary in a directory named debug.

Cargo lock file: Cargo.lock is a file that Cargo generates and updates automatically. It keeps track of the exact versions of dependencies in your project. This file is used to ensure that all builds of the project are done with the same versions of dependencies.

### 3. Run the following command to run the project.
```bash
$ cargo run
```
This command compiles the project and then runs the resulting executable all in one step, so you don’t have to run cargo build and then target/debug/hello_cargo.

### 4. Run the following command to check the project.
```bash
$ cargo check
```
This command quickly checks your code to make sure it compiles but doesn’t produce an executable.

### Note: Run the following command to build the project for release.
```bash
$ cargo build --release
```
This command compiles the project with optimizations. The resulting executable is placed in target/release instead of target/debug.
Compiled program is faster, but it takes longer to compile.
Use this when you’re ready to release your code to users or benchmark the code for performance.