# Cargo Rune
A Cargo extension for the Rune language with some helpful tools

Cargo Rune is currently a really bare bones command line interface for running rune script commands. While it successfully runs valid rune files, prints the output, and also prints the amount of time it took to run the script, it lacks basically any form of debugging. I'm still new to the language so this project was more about getting it to work than making it super useful.

Plans for the future include
- A much more robust system for debugging (like the original rune cli)
- The ability to create new rune projects by simply calling `cargo rune init` like the regular `cargo init` command

Using cargo rune is as simple as running `cargo install cargo-rune` and then running `cargo rune help` to display the help dialog. The `cargo rune` command acts basically as an alias for `cargo rune run`. You can either pass a path to `cargo rune run` or simply use the default value which is `src/main.rn`. Packaged in the sources of this repository is a simple `src/main.rn` file that prints "Hello World!" that in development you can use to test the ability for the CLI to run rune scripts.

Because crates.io project names are on a first come first serve basis I wanted to make sure there was a somewhat useful project using the "cargo-rune" name, but I recognize that ultimately this name would be better used by the actual developers of rune. With that being said, if the developer or one of the contributors to rune would like to take over either the crates.io name, this repository, or both I will happily oblige just send me an email at will@wtoll.com
