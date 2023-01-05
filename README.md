# Marcus

![Pipeline status](https://img.shields.io/github/actions/workflow/status/codingjerk/marcus/main.yml?style=flat-square)
![Code coverage](https://img.shields.io/codecov/c/github/codingjerk/marcus?style=flat-square)

> Blazing fast UCI chess engine

## Philosophy

Main goal is to be **fastest** chess engine in the world.

Fastest as in "nodes per second".

Some development rules that I follow to archive this:

- Corectness is first priority, performance is second, playing strength is third
- *All* code should be benchmarked and polished
- If there are multiple ways of doing something they both should be benchmarked

## Usage

This engine is on pretty early stage, do not use it yet.

This engine eventually will support UCI protocol.
You can read about it [here in wiki](https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&ved=2ahUKEwjo6fuo-LD8AhXk43MBHd_iCocQFnoECBsQAQ&url=https%3A%2F%2Fen.wikipedia.org%2Fwiki%2FUniversal_Chess_Interface)
and use any chess GUI interface for play with this engine.

If you just want to play with this engine,
you can challenge it [here on lichess](https://lichess.org/@/the_marcus).

## Get latest version

Every commit gets nightly release, you can download it
at [releases page](https://github.com/codingjerk/marcus/releases).

If you want to build a version for yourself,
clone this repo, `cd` into it and run:

```sh
cargo build --release
```

Dependencies is just `cargo`, rust build system and package manager.
You can install it via [rustup](https://rustup.rs).

## Contribute

Just don't. This is just a pet-project.