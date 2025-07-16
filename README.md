# BUMMER

<b>B</b>arebones <b>U</b>DP-based <b>M</b>ultiplayer <b>M</b>anager <b>E</b>ngine in <b>R</b>ust

## Getting Started

The most minimal example for running a <b>BUMMER</b> server is found in the [`minimal-server` example](https://github.com/BenjaminMassey/bummer/blob/main/examples/minimal-server). For a barebones <b>BUMMER</b> server,  all one should need to do is to modify the the `PlayerState` struct in order to comply with project needs. The rest would be handled within the game client.

For a more complete example, where a substantial-enough game client is run alongside a <b>BUMMER</b> server, see the [`minimal-macroquad` example](https://github.com/BenjaminMassey/bummer/blob/main/examples/minimal-macroquad). This is the most full example for getting started with <b>BUMMER</b>, for the time being.

For an example of a completely separate game client - written with Godot - see the WIP [`Battle Baos` project](https://github.com/BenjaminMassey/battle_baos).

## Files

The client and server need a shared authorization key to refer to. On the <b>BUMMER</b> server side, create a file `auth.key` in its root directory. Then, ingesting the text however you like, have your client use that same authorization key in your HTTP requests to the server.

In addition, <b>BUMMER</b> is configured via a settings file. Create and fill out a `bummer.toml` file in the root of your <b>BUMMER</b> server, and fill it out as desired. Here is an example:

```toml
title = "Bummer Settings"

[http]
address = "127.0.0.1"
port = 8080

[udp]
address = "127.0.0.1"
port = 8081
```

## Architecture

![architecture](<architecture.svg>)

## Contact

Feel free to contact me at benjamin.w.massey@gmail.com with any questions / inquiries.