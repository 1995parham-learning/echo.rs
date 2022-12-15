# echo.rs

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/1995parham-learning/echo.rs/test.yaml?label=test&logo=github&style=flat-square&branch=main)

## Introduction

`echo.rs` is an implementation of echo server in rust.
echo server echo back everything you send to it.
It works over TCP.

## Usage

Lunch `echo.rs` simply by

```bash
cargo run -- 127.0.0.1:1378 &
telnet 127.0.0.1 1378
```
