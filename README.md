# binsign
Simple utility for signing binaries

## How it works
Currently, it uses a OpenSSL key system that is injected into the end of the binary. We plan on adding other methods too.

## Building
1. Clone the project
```sh
git clone https://github.com/Strawberry-Foundations/binsign.git && cd binsign
```

2. Build it using cargo
```sh
cargo build
```

3. Run it using cargo
```sh
cargo run
```

## Usage

```
Simple utility for signing binaries

Usage: binsign <COMMAND>

Commands:
  gen    Generate a new certificate and save it as a file
  sign   Sign a binary file
  check  Compare a file with a certificate
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
