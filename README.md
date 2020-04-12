# yume

> An encrypted peer-to-peer IPv6 UDP messaging terminal client built with Rust

This is an early project, this readme is a draft!
A lot of things are still missing: Github actions, tests, better documentation, etc.

## Installation

```sh
cargo install yume
```

## Usage

To establish a connection between two peers, you need to start the client and provide your IPv6 and the one of the peer you want to reach:

```sh
yume 2001:3984:3989::10 2001:3984:3989::20
```

The first peer need to follow the instructions and press enter to get a new secret key.
It's up to you to share this key in a secure way, the client does not share the key with the other connected peer!

```sh
yume - An encrypted peer-to-peer IPv6 UDP messaging terminal client
Version 0.1.0

Enter secret key or press enter to generate a new one:

6uVsz9uK3KGqEfX0yg9CUpYk8TusSsnnNYmcSnmyhxwvWllFtFAqm1N7i5JYEysDELDq5EyuMYQwPPwgE2/0eg==

You can start typing!
```

The second peer, when prompted, should paste or enter the secret key manually.

### Available commands - in progress

```sh
/help
```

```sh
/quit
```

## Security

This crate uses the ChaCha20Poly1305 - Authenticated Encryption with Associated Data (AEAD) - see https://github.com/RustCrypto/AEADs/tree/master/chacha20poly1305.
Every message is encrypted with the secret key (not shared) and sent via UDP to the remote peer. The remote peer verifies the signature and the nonce to decrypt the message.