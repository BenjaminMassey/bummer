# BUMMER

## About

<b>B</b>arebones <b>U</b>DP-based <b>M</b>ultiplayer <b>M</b>anager <b>E</b>ngine in <b>R</b>ust: <b>BUMMER</b>

## Usage

To run a <b>BUMMER</b> server, one would simply need a Rust project like so:

`cargo add --git https://www.github.com/BenjaminMassey/bummer.git`
```rust
fn main() {
    bummer::start();
}
```

For a more complete example, where there is a real enough game client, and basic server interaction occurs, see the [`minimal-macroquad` example](https://github.com/BenjaminMassey/bummer/blob/main/examples/minimal-macroquad).

For an example of a completely separate game client - written with Godot - see the WIP [`Battle Baos` project](https://github.com/BenjaminMassey/battle_baos).

## Architecture

![architecture](<architecture.svg>)

## Contact

Feel free to contact me at benjamin.w.massey@gmail.com with any questions / inquiries.