# Conway's Game of Life (Rust)

I wanted to experiment with Rust a bit and I thought this would be a fun little
project. The implementation is pretty simple, here are the highlights:

- Board expands to fill the terminal automatically.
- All four sides of the board are stitched together -- if you go up past the
  board boundary then you end up on the bottom of the board. If you go too far
  left, you end up on the right. Technically this means we've formed a
  [toroid](https://en.wikipedia.org/wiki/Toroid).
- Board is saved as a list of booleans (`Vec<bool>`), not a two-dimensional
  list like you'd usually see. I don't know why I did it this way but it was
  kind of nice to work with. The only pain point was the toroid math, because
  the positions on the board have type `usize` which can't go negative. Maybe
  this was a blessing in disguise?
- Instead of using `@` and `.` to represent cell populations, I've used some
  [block elements](https://en.wikipedia.org/wiki/Block_Elements) so that each
  character actually represents four cells. For example: `▙`.
- I'm new to Rust, so my code is probably far from idiomatic.

## Getting started

Install [Rust](https://www.rust-lang.org/).

```sh
git clone https://github.com/christianbundy/rust-game-of-life.git
cd rust-game-of-life
cargo run
```

## Builds

You can `cargo build --release` and get builds for your current platform, but I
had some trouble with cross-compiling for macOS. [I rewrote this project in
Go](https://github.com/christianbundy/go-game-of-life/) while dealing with
cross-compiler problems, but eventually figured it out.

(Aside: Apparently Go comes with their own compiler that was written from
scratch. I can't decide whether I love or hate that approach.)

I'm on Linux and tried cross-compiling for macOS with:

```sh
cargo build --target x86_64-apple-darwin
```

The linker blew up in my face and then it laughed at me.

I installed [osxcross](https://github.com/tpoechtrager/osxcross) from the AUR
and then created that `./cargo/config.toml` file you see in the root directory
of this project. It kept giving me errors like this:

```
error while loading shared libraries: libxar.so.1: cannot open shared object file: No such file or directory
```

I tried a bunch of different methods of adding the library path to my Cargo
config, but none of them worked. Here's my workaround:

```sh
LD_LIBRARY_PATH="/opt/osxcross/lib/" cargo build --target x86_64-apple-darwin
```

## License

AGPL-3.0
