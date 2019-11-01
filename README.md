# solve99game-rs

> Rust solver of a nameless game that I dubbed "99game".

Implements [this solver for the 99game](https://github.com/BenWiederhake/99game/blob/master/solve.py)
in a ridiculously efficient manner.

## Table of Contents

- [Install](#install)
- [Usage](#usage)
- [TODOs](#todos)
- [NOTDOs](#notdos)
- [Contribute](#contribute)

## Install

No installation required.

## Usage

Set the parameters and go!

```
$ ./the99game.py
[FIXME: some output, then the solution for base 10]
$ ./the99game.py 12
[FIXME: some output, then the solution for base 12]
```

See [this project](https://github.com/BenWiederhake/99game) to verify that the created solutions actually are, in fact, solutions.

## Determining a pattern

[Odd degrees are easy](https://github.com/BenWiederhake/99game#odd-degrees-are-easy) and
[even degrees need expansion](https://github.com/BenWiederhake/99game#even-degrees-need-expansion).

### Experimental results:

Here I'll put the result of running it for various bases.

Have fun running it with more RAM consumption.

Sadly, the sequence "3,3,6,6,9,9,15,12,26,15" is [not known](https://oeis.org/search?q=3%2C3%2C6%2C6%2C9%2C9%2C15).
What comes next?  Who knows? :D

Since odd degrees are trivial, we might instead want to look at even degrees.
However, there's [only one very unlikely result](https://oeis.org/search?q=3,6,9,15,26) for that.
Known: `3,6,9,15,26,>30`

## TODOs

* Implement somehow
* Make it MORE efficient!
* Implement a general-purpose B-Tree that doesn't assume [`B=6` always works](https://doc.rust-lang.org/src/alloc/collections/btree/node.rs.html#42), wtf!
* Make deduce a general-purpose A* implementation for low memory?
* Make everything nicer
* Solve the world's problems

## NOTDOs

Here are some things this project will not support:
* GUI.
* AI.
* Nicer output.
* Anything networking.

## Contribute

Feel free to dive in! [Open an issue](https://github.com/BenWiederhake/solve99game-rs/issues/new) or submit PRs.
