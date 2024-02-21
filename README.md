[Rust and redis banner](./public/image.png)

This is my solution with Rust🦀 to the
["Build Your Own Redis" Challenge](https://codecrafters.io/challenges/redis).

In this challenge, I build a toy Redis clone that's capable of handling
basic commands like `PING`, `SET` and `GET`. Along the way I leart about
event loops, the Redis protocol, tokio and async Rust, and more.

## ⬇️ Dependencies
Install [🦀Cargo](https://www.rust-lang.org/tools/install) and [🤖 Just](https://github.com/casey/just) 

```bash
sudo apt install cargo
cargo install just
```

## 💣 Usage

| Description | Command |
| ----------- | ------- |
| 🧪 Run tests   | `just test` |
| 👁️ Run tests in watch mode   | `just test_watch` |
| 🚀 Run server  | `just run`  |
