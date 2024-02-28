# rsTimer

Command line application for timing you Rubik's cube solves.

## Installation:

You have to compile it yourself, but that shouldn't be a problem. Only thing
you need is `cargo`:

```
cargo build -r
```

After its done compiling, you can start it in `./target/release/rstimer`

## Usage:

Open timer *(opens session picker to choose, which session to open)*:
```
./rstimer
```

You can check other usage cases in help:
```
./rstimer -h
```

## Technologies:

Since this was my **first project** in Rust, I used more libraries then I had
to. You can check all the libraries in `Cargo.toml` file.

Just to mention some libraries I used
[termint](https://github.com/Martan03/termint) for printing (my library :) )
and [crossterm](https://github.com/crossterm-rs/crossterm) for handling user
input.

## Detailed description:
TODO

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [rsTimer](https://github.com/Martan03/rsTimer)
- **Author website:** [martan03.github.io](https://martan03.github.io)
