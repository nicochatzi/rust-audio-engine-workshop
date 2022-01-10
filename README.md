# Rust Workshops to build a synth for multiple platforms

This workshop will cover the process of creating an audio engine in Rust
and using that engine as a static library in different environments.
The engine is a 4-operator FM synth.

This project aims to show how to write a single Rust library that can
be consumed by a:
* C/C++ desktop application
* C/C++ embedded application
* WASM-capable environment
* Python script

## Getting Started

To get started, run:

```
$ curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/nicochatzi/rust-audio-engine-workshop/master/template.sh | sh
```

This will check you have the required tools, then clone the repo and
place you at a point in the git history where the project is setup and ready for you
to start the workshop!

If you're joining the workshop at some other point, run the previous command, then
run one of the following command:

* for part 2 : `git checkout HASH`
* for part 3 : `git checkout HASH`
* for part 4 : `git checkout HASH`
* for part 5 : `git checkout HASH`
* for part 6 : `git checkout HASH`

### Requirements

* git
* Rust
* Cargo
* CMake
* Node/NPM
* Python 3
* Poetry

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

## Part 4 : Offline analysis : Python

For the fourth workshop, we will create Python bindings
to allow in-depth offline analysis of the Audio Engine
using Python's fantastic tooling! Specifically using Jupyter
Notebooks.

## Part 5 : More Core : Rust

For the fifth workshop, we will be extending the core engine
with another Rust crate that will handle saving and recalling patches.

## Part 6 : Embedded : Rust

For the sixth workshop, we will be compiling the engine
for embedded platforms.