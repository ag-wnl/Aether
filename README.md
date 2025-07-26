# Aether

light weight barebones OS, implemented in Rust programming language.

## Run Locally

We have created a custom triple-target which has no underlying OS.
To make experimental features work, use Rust nightly

```bash
  rustup override set nightly
```

To compile (cross-compile) the project project.

```bash
  cargo build --target x86_64_aether.json
```

To use existing targets, you can simply install them and set
them as the compilation target while using cargo build.

```bash
  rustup target add thumbv7em-none-eabihf
```
