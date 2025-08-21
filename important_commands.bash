# pwd command : tells the current working directory in which we are working.
command : pwd

# ls command : list all the files and folder within the current working directory.
command : ls

# cd command : it is used to change the directory
command : cd

#command to check the rust version : 
command : rustc --version 

# rustup that helps us to update , install and uninstall the rust compiler from the computer.
command : rustup  # this shows multiple functions rustup can perform we can chose from the options list.

# rustup command to uinstall the rust : 
command : rustup self uninstall

# rustup command to open the rust official documentation : 
command : rustup doc

# command to create a new rust project : 
command : cargo new project_name


# compiling rust program from the terminal : 
command : cd src -- > if outside the src 
        : rustc main.rs
        : ./main -- > to run the executable

# command to format the rust code : 
command : rustfmt main.rs

# comman to check the system spec according to that .exe file is created
command : file main

# global format command  , it formats all the rust files in the directory ,
# try to apply from top directory or parent directory
command : cargo fmt

/*
Running `cargo fmt` works only when executed from the top-level project directory
— i.e., the directory that contains the `Cargo.toml` file.

This is because `cargo fmt` uses the project's metadata and structure defined in `Cargo.toml`
to locate and format all Rust source files correctly.

If you want to format multiple Rust files in one project (e.g., multiple binaries),
your project directory should follow a structure like this:

my_project/
├── Cargo.toml           <-- Required: tells Rust it's a project root
├── src/
│   └── main.rs          <-- Default binary entry point
└── src/bin/
    ├── tool1.rs         <-- Additional binaries
    └── tool2.rs

With this layout, running `cargo fmt` from the `my_project/` directory
will format:
  - main.rs
  - tool1.rs
  - tool2.rs
  - any other .rs files under the project

⚠️ Note:
- Running `cargo fmt` from a subdirectory (like `src/`) may only format local files.
- Without a `Cargo.toml` file in the top-level directory, `cargo fmt` will fail.

Conclusion:
📍 Always run `cargo fmt` from the root project directory where `Cargo.toml` exists
to format all Rust files at once.
*/


command : cargo build  -> to create unoptimized build
command : cargo build --release  -> it creates the optimized build and ready for the production
command : cargo clean   -> It deletes all the target folder created in the program and gives us a fresh start, but it runs inside the rust project only because it searches 
for the .toml file, so simply running the clean command in the parent directory will not remove the target folder from the each of the 
child rust project sub-directories
command : ./target/release/<project_name> -> to run the build
command : cargo run -> this command build+run the code.This creates unoptimized build
command : cargo run --quiet ->this command does similar thing that cargo run does but not shows any intermediates results
-> it directly shows the output
command : 
