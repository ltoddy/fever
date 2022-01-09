# fever

Initialize project using templates.

Prerequisites:

1. Git (`brew install git`)
2. rust (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
3. fever (`cargo install fever`)

### Usage

#### Rust

1. Initialize rust project

```shell
$ mkdir rust-example && cd rust-example
$ fever rust init --description "Just a rust project"
$ tree .
.
├── Cargo.toml
├── LICENSE-APACHE
├── LICENSE-MIT
├── README.md
├── rust-toolchain
├── rustfmt.toml
└── src
    └── main.rs

1 directory, 7 files
```

2. Create rust project

```shell
$ fever rust new rust-example --description "Rust project"
```
