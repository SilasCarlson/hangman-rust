# Overview

I am trying to learn more about lower-level languages and how they interact with the
computer in order to further my learning, but also to increase my skill-set. My goals 
with this software is to further my learning regarding lower-level languages.

The purpose of this software is to help me learn Rust and how more complex applications 
can be built using it. Also, the purpose is to explore how OOP is implemented in a 
language that does not necessarily support common OOP practices (such as inheritance, 
polymorphism, etc.). Another purpose was to explore and understand how Cargo works and
how it is used to build applications and make development faster/easier.

The software is a simple hangman game in the console, in which the
player can guess letters until they guess the phrase. The game uses a simple state 
management system in order to manage what part of the game the player is currently on.
The software uses structs and implementations in order to run and is spread across
various files, which are then included in the `main.rs` file.

[Software Demo Video](https://www.youtube.com/watch?v=m0wTgUS0L4Y)

# Development Environment

The project was developed solely using Rust, and it's building tool: Cargo. In order to 
build the project yourself and run it you would have to clone the repository and then 
run the project using cargo.
```
cargo run
```

To build the executable you would build it with cargo as well.
```
cargo build
```

The project was developed in Visual Studio Code, using the Rust extension.
The following libraries were used in this project:
* [ClearScreen](https://docs.rs/clearscreen/latest/clearscreen/) - To clear the
console screen.
* [Serde](https://serde.rs/) - To deserialize the data from the CSV file into a rust 
struct flawlessly.
* [CSV](https://docs.rs/csv/latest/csv/) - To load and read the CSV file for the 
phrases.
* [Rand](https://docs.rs/rand/latest/rand/) - To generate a random number and to
generate a random index to grab a random phrase.

# Useful Websites

- [Rust Documentation](https://doc.rust-lang.org/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [JetBrains - How to Learn Rust in 2025](https://blog.jetbrains.com/rust/2024/09/20/how-to-learn-rust/)

# Future Work

- A GUI, rather than a CLI
- Saving/loading the player's state (how many wins, etc.)
- Transitions between states
- Make it so players can not repeat a phrase