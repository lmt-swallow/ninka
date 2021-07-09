# Ninka

[![Run tests](https://github.com/lmt-swallow/ninka/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/lmt-swallow/ninka/actions/workflows/test.yml) [![Run lint](https://github.com/lmt-swallow/ninka/actions/workflows/lint.yml/badge.svg?branch=main)](https://github.com/lmt-swallow/ninka/actions/workflows/lint.yml)

This repository has core implementations on Ninka, a re-implementation of Zanzibar.

## How to run Ninka locally

You can run Ninka program with the following command(s):

```sh
cargo run -- help
```

## How to install Ninka

You can install Ninka by the following command(s):

```sh
cargo install --path . --force
```

After you have successfully installed Ninka, you can see help as follows:

```sh
ninka help
```

You can install shell completions as follows:

```sh
# in bash
eval "$(ninka completion bash)"

# in fish
ninka completion fish | source
```

## How to run tests locally

You can run tests with the following command(s):

```sh
cargo test
```
