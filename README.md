# entropia

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/clementi/entropia/CI)](https://github.com/clementi/entropia/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/entropia)](https://crates.io/crates/entropia)

Calculate the information entropy of some text

## Installation

You can install entropy either by using Cargo, or by downloading a binary from
the Releases page.

### Cargo

Run this in your shell:

```sh
$ cargo install entropia
```

### Binaries

See the [Releases](https://github.com/clementi/entropia/releases) page.

## Usage

Entropy takes no arguments. It captures text on stdin and reports the
information entropy of that text. For example:

```
$ cat file | entropy
2362.4587346565673

$ secret token | entropy
138.9089067264194

$ echo "Hello world" | entropy
69.68825906469124

$ entropy
Hello world
69.68825906469124
```
