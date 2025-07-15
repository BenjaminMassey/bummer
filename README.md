# BUMMER

## About

<b>B</b>arebones <b>U</b>DP-based <b>M</b>ultiplayer <b>M</b>anager <b>E</b>ngine in <b>R</b>ust: <b>BUMMER</b>

## Usage

The most minimal example for running a <b>BUMMER</b> server is found in the [`minimal-server` example](https://github.com/BenjaminMassey/bummer/blob/main/examples/minimal-server). Note that it will note be very useful, since the sending and receiving of some useful `PlayerState` is primarily the purpose of <b>BUMMER</b>. Here is that minimal example:

`cargo add --git https://www.github.com/BenjaminMassey/bummer.git`

`cargo add serde --features derive`

`echo abc123 > auth.key`

```rust
#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct PlayerState;

fn main() {
    bummer::start(PlayerState);
}
```

Note that the client and server need a shared authorization key to refer to. On the <b>BUMMER</b> server side, create a file `auth.key` in its root directory. Then, ingesting the text however you like, have your client use that same authorization key in your HTTP requests to the server.

For a more complete example, where there is a real enough game client, and basic server interaction occurs, see the [`minimal-macroquad` example](https://github.com/BenjaminMassey/bummer/blob/main/examples/minimal-macroquad).

For an example of a completely separate game client - written with Godot - see the WIP [`Battle Baos` project](https://github.com/BenjaminMassey/battle_baos).

## Architecture

![architecture](<architecture.svg>)

## Contact

Feel free to contact me at benjamin.w.massey@gmail.com with any questions / inquiries.