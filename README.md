# Rust Workshops to build a synth for multiple platforms

This workshop will cover the process of creating a simple tone player in Rust
and using that as a library in different environments.

This project aims to show how to write a single Rust library that can
be consumed by a:
* C/C++ desktop application
* WASM-capable environment

## Getting Started

To get started, run:

```
$ curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/nicochatzi/rust-audio-engine-workshop/master/template.sh | sh
```

This will check you have the required tools, then clone the repo and
place you at a point in the git history where the project is setup and ready for you
to start the workshop!

### Requirements

* git
* Rust
* Cargo
* CMake
* Node/NPM

## Part 1 : Core engine : Rust

For the first workshop, we will be building the core engine
that can be controlled through MIDI with minimal TUI feedback.

## Part 2 : JUCE App : C/C++

For the second workshop, we will embedded the engine into
a cross-platform JUCE plugin. The focus here is learn about
Rust FFI and how to rebuild our library for each platform.

## Part 3 : Web App : JS/TS && WASM

For the third workshop, we will embedded the engine into
a web app using React and WASM.

