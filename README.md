# Read readrust.net on your command line

This is a small tool that reads the readrust.net json
feed and prints it out as a table. It can also show
you a count of collected posts.

It is intended as an educational tool.

## Usage

This can be either run directly through cargo or by calling the binary.

```sh
$ cargo run -- --help
```

```sh
$ cargo build && target/debug/readrust --help
```

## Build requirements

### Ubuntu

```
$ sudo apt-get install pkg-config libssl-dev
```

### Building

```
$ cargo build [--release]
```

