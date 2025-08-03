# dll_proxy-rs
Dll proxy in rust

# Concept

Implementation of dll proxying in `windows` using a `.def` file to prevent stub generation and using full dll path to facilitate COM Hijacking.

# Compiling

## Without debug info
```bash
$ make
```

## With debug info
```bash
$ make debug
```
