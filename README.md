[![torophy](https://img.shields.io/crates/v/torophy)](https://crates.io/crates/torophy)
[![license](https://img.shields.io/crates/l/torophy)](https://www.apache.org/licenses/LICENSE-2.0.txt)
[![downloads](https://img.shields.io/crates/d/torophy)](https://crates.io/crates/torophy)

# `torophy`
2D physics engine over a toroidal space.
The aim of this project is to test and experiment with physics over a toroidal space.

## Features
- Toroidal space physics.
- Particle physics.
- Rigid body collision (circles).
- Fast: focus in to make the collisions as fast as possible.
- Easy to use: Simple API to make your own experiments really easy and fast.

## Examples
<p align="center">
  <img src="https://media.giphy.com/media/gjwqcqS4P2pivrJVeA/giphy.gif"/>
  <img src="https://media.giphy.com/media/kbXIqEwfGRt1OpXWKj/giphy.gif"/>
  <img src="https://media.giphy.com/media/gLo1PJjumwbhFFErOg/giphy.gif"/>
  <img src="https://media.giphy.com/media/J1LOOFj2gOneEvW0YL/giphy.gif"/>
</p>

You can see more examples in the `examples` folder.

## Test it!

### Install from cargo repositories
It is necessary to have `rust` language installed (you can install from [rustup.rs](https://rustup.rs/))

```sh
cargo install torophy --examples
```

if you have `~/.cargo/bin` in PATH you only need to run the example by its name.

### From sources
```sh
git clone https://github.com/lemunozm/torophy.git
cargo run --example basic --release
```

## Getting started
Add to your `Cargo.toml`
```sh
torophy = "0.1"
```

You can change `basic` name for any other example name found in the `examples` folder.

## Contribute!
Any help is welcome!
If you have any awesome idea for a new experiment or feature, please, create an issue or a pull request.
