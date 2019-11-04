# solve99game-rs

> Rust solver of a nameless game that I dubbed "99game".

Implements [this solver for the 99game](https://github.com/BenWiederhake/99game/blob/master/solve.py)
in a ridiculously memory-efficient manner.

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
$ cargo run
   Compiling solve99game-rs v0.1.0 (/home/eispin/workspace/solve99game-rs)
    Finished dev [unoptimized + debuginfo] target(s) in 1.02s
     Running `target/debug/solve99game-rs`
Running with BASE = 12
Initial board = FullBoard([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 1, 1, 1, 2, 1, 3, 1, 4, 1, 5, 1, 6, 1, 7, 1, 8, 1, 9, 1, 10, 1, 11])
Hash = [138, 185, 94, 42, 199, 246, 164, 125, 4, 149, 208, 117, 142, 13, 45, 119]
Each node needs a total of 48.00 bytes (12 incremental, 36 seen).
The 'seen' HashMap is responsible for 36.00 bytes of that, instead of 21+eps.
IncrementalNode is 8 instead of 4 + 1 = 5
Theoretical minimum is 4 + 5 + 16 + 1 + 4 = 30
---------------------------------------------
At least  0 turns,       0 steps,       0 open,       1 closed, looking at turn  0,   0.0+  0.0 MiB in use
At least 18 turns,       1 steps,       9 open,       2 closed, looking at turn  1,   0.0+  0.0 MiB in use
At least 18 turns,       2 steps,      16 open,       3 closed, looking at turn  2,   0.0+  0.0 MiB in use
At least 18 turns,       3 steps,      20 open,       4 closed, looking at turn  3,   0.0+  0.0 MiB in use
At least 18 turns,       4 steps,      22 open,       5 closed, looking at turn  4,   0.0+  0.0 MiB in use
At least 18 turns,       5 steps,      23 open,       6 closed, looking at turn  5,   0.0+  0.0 MiB in use
<Some lines omitted>
At least 33 turns, 7090000 steps, 7089919 open, 7090001 closed, looking at turn 23, using 595.01-1190.03 MiB
At least 33 turns, 7100000 steps, 7099913 open, 7100001 closed, looking at turn 19, using 595.85-1191.70 MiB
At least 33 turns, 7110000 steps, 7109954 open, 7110001 closed, looking at turn 25, using 596.69-1193.39 MiB
At least 33 turns, 7120000 steps, 7119936 open, 7120001 closed, looking at turn 23, using 597.53-1195.06 MiB
At least 33 turns, 7130000 steps, 7129899 open, 7130001 closed, looking at turn 17, using 598.37-1196.74 MiB
At least 33 turns, 7140000 steps, 7139942 open, 7140001 closed, looking at turn 22, using 599.21-1198.42 MiB
Done after 7144352 steps.
=== Can win after 33 turns! ===
[9,2,>, 'expand', 1,4,>, 10,3,v, 0,3,v, 9,3,>, 8,2,>, 1,2,v, 0,2,v, 9,4,>, 8,4,>, 7,4,>, 6,4,>, 5,4,>, 4,4,>, 1,1,>, 10,0,v, 9,1,>, 8,1,>, 7,1,>, 6,1,>, 5,1,>, 4,1,>, 8,0,v, 7,0,v, 6,0,v, 5,0,v, 4,0,v, 2,0,v, 0,0,v, 9,0,>, 3,0,v, 1,0,>]
2894.99user 5.05system 48:38.90elapsed 99%CPU (0avgtext+0avgdata 702896maxresident)k
0inputs+0outputs (0major+199168minor)pagefaults 0swaps
```

See [this project](https://github.com/BenWiederhake/99game) to verify that the created solutions actually are, in fact, solutions.

### Observations

Note that the estimate of "asympoticially 48 bytes per entry" seems to hold true; it actually looks like 50.
This means that A* can be run for a ridiculous amount of nodes, given enough RAM.

I'm using `u32` for indices everywhere.  This is because you'll need
at *absolute minimum* 84 GiB (90 GB) of RAM before any index can reach `0xffffffff`
(i.e., `u32::max_value()`).

Given the above numbers, on average 4894 nodes get generated (not closed) per second, or 847 MiB/h.
So in order to fill, say, 16 GiB of RAM, I would wait at least 20 hours.

## Determining a pattern

[Odd degrees are easy](https://github.com/BenWiederhake/99game#odd-degrees-are-easy) and
[even degrees need expansion](https://github.com/BenWiederhake/99game#even-degrees-need-expansion).

### Experimental results:

These are the result of running it for various bases.

```
Base	Len	Solution
2	3	[0,0,v, 'expand', 0,0,v]
3	3	[0,2,>, 0,1,>, 0,0,>]
4	6	[1,2,>, 1,1,>, 2,0,>, 1,0,>, 'expand', 0,0,v]
5	6	[2,2,>, 1,1,>, 0,1,v, 3,1,>, 1,0,>, 0,0,>]
6	9	[3,2,>, 1,1,v, 0,1,>, 4,0,v, 3,0,v, 2,0,>, 1,0,>, 'expand', 0,0,v]
7	9	[4,2,>, 3,1,v, 2,1,>, 0,1,>, 2,0,>, 1,0,>, 0,0,v, 5,1,>, 5,0,>]
8	15	[5,2,>, 1,1,>, 6,0,v, 2,0,v, 1,2,>, 0,0,v, 5,0,>, 'expand', 4,2,>, 0,2,>, 5,1,>, 4,1,>, 4,0,v, 3,0,>, 1,0,>]
9	12	[6,2,>, 4,1,v, 1,1,>, 0,1,v, 7,1,>, 6,1,>, 5,1,>, 3,1,>, 3,0,>, 2,0,>, 1,0,>, 0,0,>]
10	26	[8,0,v, 0,0,v, 'expand', 3,4,v, 1,4,v, 8,3,v, 4,3,v, 0,3,v, 7,2,v, 6,2,v, 5,3,>, 3,3,>, 5,2,>, 4,2,>, 2,2,v, 1,2,>, 6,1,v, 4,1,v, 1,1,>, 7,0,>, 6,0,>, 5,0,>, 4,0,>, 3,0,>, 2,0,>, 1,0,>]
11	15	8,2,>, 6,1,v, 5,1,v, 4,1,v, 1,1,>, 0,1,v, 9,1,>, 8,1,>, 7,1,>, 3,1,>, 4,0,>, 3,0,>, 2,0,>, 1,0,>, 0,0,>
12	33	9,2,>, 'expand', 1,4,>, 10,3,v, 0,3,v, 9,3,>, 8,2,>, 1,2,v, 0,2,v, 9,4,>, 8,4,>, 7,4,>, 6,4,>, 5,4,>, 4,4,>, 1,1,>, 10,0,v, 9,1,>, 8,1,>, 7,1,>, 6,1,>, 5,1,>, 4,1,>, 8,0,v, 7,0,v, 6,0,v, 5,0,v, 4,0,v, 2,0,v, 0,0,v, 9,0,>, 3,0,v, 1,0,>
```

Have fun running it with more time and RAM.

Sadly, the sequence "3,3,6,6,9,9,15,12,26,15,33" is [not known](https://oeis.org/search?q=3,3,6,6,9,9,15).
What comes next?  Who knows? :D

Since odd degrees are trivial, we might instead want to look at even degrees:
But such a sequence is also [not known](https://oeis.org/search?q=3,6,9,15,26,33).

## TODOs

* Make it EVEN MORE efficient?!
* Maybe optimize a little bit for speed?
* When running out of RAM: Look into disk-based solutions
* Maybe a LRU cache for `all_set`-index to `FullBoard` translation?
* Implement a general-purpose B-Tree that doesn't assume that [`B=6` always works](https://doc.rust-lang.org/src/alloc/collections/btree/node.rs.html#42), wtf!
  This could be used for `seen` for a much smoother memory consumption.
* Solve the world's problems

## NOTDOs

Here are some things this project will not support:
* GUI.
* AI.
* Nicer output.
* Anything networking.

## Contribute

Feel free to dive in! [Open an issue](https://github.com/BenWiederhake/solve99game-rs/issues/new) or submit PRs.
