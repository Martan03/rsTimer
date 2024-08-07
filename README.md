# rsTimer

Rubik's cube TUI timer in Rust 🦀

![image](https://github.com/user-attachments/assets/2e54d0f4-01c3-4dd5-8ea7-496b63c2b086)

## Installation:

You have to compile it yourself, but that shouldn't be a problem. Only thing
you need is `cargo`:

```
cargo build -r
```

After it's done compiling, you can start it in `./target/release/rstimer`

## Usage:

You can start the timer like this *(session picker will open)*:
```
./rstimer
```

Or you can **skip** the session picker by specifying the session:
```
./rstimer <session_name>
```

You can check other usage in help:
```
./rstimer -h
```

## Detailed description:

### Sessions:

**rsTimer** contains sessions. You can create as many sessions as you want.
Each sessions has its scramble type *(currently only 2x2x2, 3x3x3 and 4x4x4
are supported)*.

When you run **rsTimer** with no arguments, the session picker will open and you 
will see all created sessions. In order to time the solves, you need to select 
a session. You can use the `Up/Down arrow` keys to scroll through the list and
press `Enter` to open currently selected session.

![image](https://github.com/user-attachments/assets/6d2a9223-a864-4371-955d-1b66c152797b)

### Scrambles:
**rsTimer** currently supports scrambles for **2x2x2**, **3x3x3** and **4x4x4** 
only. More scramble types will be added in the future.

**Note**: scrambles are **not** guaranteed to be WCA

### Timing:
When you run **rsTimer** and open a session, you can start timing your
solves. There's scramble on top, which is based on the set scramble type of the
session. To start the timer, you have to press a `Space`. When you're done, you
can press `Space` again to stop the timer. New scramble will be generated and
you will see your time.

On the left side of the timer there are listed previous solves. You can scroll
through them by using `Up/Down arrow` keys. You can also delete currently 
selected time by pressing `Del` key.

You can also change the font of the timer *(more fonts will be added later)*
by pressing `Left/Right arrow` key. The set font is saved and used the next 
time you use the timer.

![image](https://github.com/user-attachments/assets/2e54d0f4-01c3-4dd5-8ea7-496b63c2b086)

### Other keybinds (work when timer is not running):
`Esc/q`: closes **rsTimer**
`s`: switches to the session picker

## Technologies:

This was my **first project** in Rust 🦀, but since then I've refactored the
code quite a bit.

Some of the libraries I used:
- [termint](https://github.com/Martan03/termint): TUI
- [crossterm](https://github.com/crossterm-rs/crossterm): handling input
- [dirs](https://crates.io/crates/dirs): accessing `~/.config` folder
- [serde](https://crates.io/crates/serde): saving to JSON files

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [rsTimer](https://github.com/Martan03/rsTimer)
- **Author website:** [martan03.github.io](https://martan03.github.io)
