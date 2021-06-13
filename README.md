# echo.rs

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/1995parham/echo.rs/test?label=test&logo=github&style=flat-square)

## Introduction

`echo.rs` is an implementation of echo server in rust.
echo server echo back everything you send to it.
It works over TCP.

## Usage

Lunch `echo.rs` simply by

```sh
cargo run -- -l 127.0.0.1:1378 &
telnet 127.0.0.1 1378
```
