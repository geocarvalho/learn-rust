# [Introdução a linguagem Rustlang](https://www.youtube.com/playlist?list=PLjSf4DcGBdiGCNOrCoFgtj0KrUq1MRUME)
## Rodar um script
```bash
$ cd /Users/geocarvalho/Documents/repos/aprenda-rust/
$ rustc hello.rs
$ cargo new proj
$ cd proj
$ cargo build
$ vim src/main.rs
$ cargo run
```

## Instalar o compilador automatico (problematico)
```bash
$ cargo install cargo-watch
$ cargo watch -x run
```