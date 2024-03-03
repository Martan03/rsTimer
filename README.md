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
to *(I refactored it a bit already)*. You can check all the libraries in
`Cargo.toml` file.

Just to mention some libraries I used
[termint](https://github.com/Martan03/termint) for printing (my library :) )
and [crossterm](https://github.com/crossterm-rs/crossterm) for handling user
input.

## Detailed description:
### Sessions:
**rsTimer** contains sessions. You can create as many sessions as you want.
Each sessions has its scramble type.

When you start **rsTimer** without arguments, session picker will be opened
and you will see all the created sessions. To be able to time solves, you need
to select a session. You can use `Up/Down arrow` keys to navigate the list and
press `Enter` to open currently selected session.

### Scrambles:
Currently only supported scramble types are **2x2x2**, **3x3x3** and **4x4x4**.
More scramble types will be added in the future. Note that the scrambles
**aren't** WCA. This I might focus on in the future as well.

### Timing:
When you start **rsTimer** and open a session, you can start timing your
solves. There's scramble on top, which is based on set scramble type of the
session. You can start timing a solve by pressing `Space`. After that the
timer will be started and you can start solving the cube. When you're done, you
can press `Space` again to stop the timer. New scramble will be generated and
you will see your time.

### Stats:
Currently you can only display all times in the current session, but I'm
planning to add more stats in the future.

You can open the stats from the page with the generated scramble. To open them
you can press `Tab`. To exit stats you can simply press `Tab` again. When you
press `Delete`, currently selected time will be deleted.

### Other keybinds:
`Esc` closes **rsTimer** (note that timer can't be running)

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [rsTimer](https://github.com/Martan03/rsTimer)
- **Author website:** [martan03.github.io](https://martan03.github.io)
