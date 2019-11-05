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
$ cargo build && /usr/bin/time cargo run
   Compiling solve99game-rs v0.1.0 (/home/eispin/workspace/solve99game-rs)
    Finished dev [unoptimized + debuginfo] target(s) in 1.18s
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/solve99game-rs`
Running with BASE = 12
Initial board = FullBoard([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 1, 1, 1, 2, 1, 3, 1, 4, 1, 5, 1, 6, 1, 7, 1, 8, 1, 9, 1, 10, 1, 11])
Hash = [138, 185, 94, 42, 199, 246, 164, 125, 4, 149, 208, 117, 142, 13, 45, 119]
Each node needs a total of 48.00 bytes (12 incremental, 36 seen).
The 'seen' HashMap is responsible for 36.00 bytes of that, instead of 21+eps.
IncrementalNode is 8 instead of 4 + 1 = 5
Theoretical minimum is 4 + 5 + 16 + 1 + 4 = 30
---------------------------------------------
Need  0 turns,       0 steps,       0 open,       1 closed, looking at turn  0,   0.0+  0.0 MiB in use
Need 18 turns,       1 steps,       9 open,       2 closed, looking at turn  1,   0.0+  0.0 MiB in use
Need 18 turns,       2 steps,      16 open,       3 closed, looking at turn  2,   0.0+  0.0 MiB in use
Need 18 turns,       3 steps,      20 open,       4 closed, looking at turn  3,   0.0+  0.0 MiB in use
<Some lines omitted>
Need 33 turns, 7090000 steps, 7089919 open, 7090001 closed, looking at turn 23, 270.5+324.6 MiB in use
Need 33 turns, 7100000 steps, 7099913 open, 7100001 closed, looking at turn 19, 270.8+325.0 MiB in use
Need 33 turns, 7110000 steps, 7109954 open, 7110001 closed, looking at turn 25, 271.2+325.5 MiB in use
Need 33 turns, 7120000 steps, 7119936 open, 7120001 closed, looking at turn 23, 271.6+325.9 MiB in use
Need 33 turns, 7130000 steps, 7129899 open, 7130001 closed, looking at turn 17, 272.0+326.4 MiB in use
Need 33 turns, 7140000 steps, 7139942 open, 7140001 closed, looking at turn 22, 272.4+326.8 MiB in use
Done after 7144352 steps.
=== Can win after 33 turns! ===
[9,2,>, 'expand', 1,4,>, 10,3,v, 0,3,v, 9,3,>, 8,2,>, 1,2,v, 0,2,v, 9,4,>, 8,4,>, 7,4,>, 6,4,>, 5,4,>, 4,4,>, 1,1,>, 10,0,v, 9,1,>, 8,1,>, 7,1,>, 6,1,>, 5,1,>, 4,1,>, 8,0,v, 7,0,v, 6,0,v, 5,0,v, 4,0,v, 2,0,v, 0,0,v, 9,0,>, 3,0,v, 1,0,>]
2892.35user 4.18system 48:33.47elapsed 99%CPU (0avgtext+0avgdata 703760maxresident)k
0inputs+0outputs (0major+179891minor)pagefaults 0swaps
```

See [this project](https://github.com/BenWiederhake/99game) to verify that the created solutions actually are, in fact, solutions.

### Observations

Note that the estimate of "asympoticially 48 bytes per entry" seems to hold true; it actually looks like 50.
This means that A* can be run for a ridiculous amount of nodes, given enough RAM.

I'm using `u32` for indices everywhere.  This is because you'll need
at *absolute minimum* 84 GiB (90 GB) of RAM before any index can reach `0xffffffff`
(i.e., `u32::max_value()`).

Given the above numbers, on average 4900 nodes get generated (not closed) per second, or 850 MiB/h.
So in order to fill, say, 16 GiB of RAM, I would wait at least 20 hours.

Also, for whatever reason it is *faster* for `BASE = 14`:

```
$ cargo build && /usr/bin/time cargo run
   Compiling solve99game-rs v0.1.0 (/home/eispin/workspace/solve99game-rs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.92s
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/solve99game-rs`
Running with BASE = 14
Initial board = FullBoard([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 1, 1, 1, 2, 1, 3, 1, 4, 1, 5, 1, 6, 1, 7, 1, 8, 1, 9, 1, 10, 1, 11, 1, 12, 1, 13])
Hash = [44, 34, 19, 206, 136, 83, 192, 91, 10, 194, 34, 63, 226, 11, 37, 166]
Each node needs a total of 48.00 bytes (12 incremental, 36 seen).
The 'seen' HashMap is responsible for 36.00 bytes of that, instead of 21+eps.
IncrementalNode is 8 instead of 4 + 1 = 5
Theoretical minimum is 4 + 5 + 16 + 1 + 4 = 30
---------------------------------------------
Need  0 turns,       0 steps,       0 open,       1 closed, looking at turn  0,   0.0+  0.0 MiB in use
Need 21 turns,       1 steps,       8 open,       2 closed, looking at turn  1,   0.0+  0.0 MiB in use
Need 21 turns,       2 steps,      14 open,       3 closed, looking at turn  2,   0.0+  0.0 MiB in use
<Some lines omitted>
Need 39 turns, 1120000 steps, 1119960 open, 1120001 closed, looking at turn  5,  42.7+ 51.3 MiB in use
Need 39 turns, 1130000 steps, 1129958 open, 1130001 closed, looking at turn  8,  43.1+ 51.7 MiB in use
Need 39 turns, 1140000 steps, 1139991 open, 1140001 closed, looking at turn 11,  43.5+ 52.2 MiB in use
Need 39 turns, 1150000 steps, 1150040 open, 1150001 closed, looking at turn 18,  43.9+ 52.6 MiB in use
Done after 1152208 steps.
=== Can win after 39 turns! ===
[11,2,>, 'expand', 1,4,>, 12,3,v, 0,3,v, 11,3,>, 10,2,>, 1,2,v, 0,2,v, 11,4,>, 10,4,>, 9,4,>, 8,4,>, 7,4,>, 6,4,>, 5,4,>, 4,4,>, 1,1,>, 12,0,v, 11,1,>, 10,1,>, 9,1,>, 8,1,>, 7,1,>, 6,1,>, 5,1,>, 4,1,>, 10,0,v, 9,0,v, 8,0,v, 7,0,v, 6,0,v, 5,0,v, 4,0,v, 2,0,v, 0,0,v, 11,0,>, 3,0,v, 1,0,>]
529.01user 0.73system 8:52.45elapsed 99%CPU (0avgtext+0avgdata 183240maxresident)k
0inputs+0outputs (0major+9082minor)pagefaults 0swaps
```

## Determining a pattern

[Odd degrees are easy](https://github.com/BenWiederhake/99game#odd-degrees-are-easy) and
[even degrees need expansion](https://github.com/BenWiederhake/99game#even-degrees-need-expansion).

### Experimental results:

These are the result of running it for various bases.

```
Base	Len	Solution
2	3	0,0,v, 'expand', 0,0,v
3	3	0,2,>, 0,1,>, 0,0,>
4	6	1,2,>, 1,1,>, 2,0,>, 1,0,>, 'expand', 0,0,v
5	6	2,2,>, 1,1,>, 0,1,v, 3,1,>, 1,0,>, 0,0,>
6	9	3,2,>, 1,1,v, 0,1,>, 4,0,v, 3,0,v, 2,0,>, 1,0,>, 'expand', 0,0,v
7	9	4,2,>, 3,1,v, 2,1,>, 0,1,>, 2,0,>, 1,0,>, 0,0,v, 5,1,>, 5,0,>
8	15	5,2,>, 1,1,>, 6,0,v, 2,0,v, 1,2,>, 0,0,v, 5,0,>, 'expand', 4,2,>, 0,2,>, 5,1,>, 4,1,>, 4,0,v, 3,0,>, 1,0,>
9	12	6,2,>, 4,1,v, 1,1,>, 0,1,v, 7,1,>, 6,1,>, 5,1,>, 3,1,>, 3,0,>, 2,0,>, 1,0,>, 0,0,>
10	26	8,0,v, 0,0,v, 'expand', 3,4,v, 1,4,v, 8,3,v, 4,3,v, 0,3,v, 7,2,v, 6,2,v, 5,3,>, 3,3,>, 5,2,>, 4,2,>, 2,2,v, 1,2,>, 6,1,v, 4,1,v, 1,1,>, 7,0,>, 6,0,>, 5,0,>, 4,0,>, 3,0,>, 2,0,>, 1,0,>
11	15	8,2,>, 6,1,v, 5,1,v, 4,1,v, 1,1,>, 0,1,v, 9,1,>, 8,1,>, 7,1,>, 3,1,>, 4,0,>, 3,0,>, 2,0,>, 1,0,>, 0,0,>
12	33	9,2,>, 'expand', 1,4,>, 10,3,v, 0,3,v, 9,3,>, 8,2,>, 1,2,v, 0,2,v, 9,4,>, 8,4,>, 7,4,>, 6,4,>, 5,4,>, 4,4,>, 1,1,>, 10,0,v, 9,1,>, 8,1,>, 7,1,>, 6,1,>, 5,1,>, 4,1,>, 8,0,v, 7,0,v, 6,0,v, 5,0,v, 4,0,v, 2,0,v, 0,0,v, 9,0,>, 3,0,v, 1,0,>
13	18	10,2,>, 8,1,v, 6,1,v, 4,1,v, 1,1,>, 0,1,v, 11,1,>, 10,1,>, 9,1,>, 7,1,>, 5,1,>, 3,1,>, 5,0,>, 4,0,>, 3,0,>, 2,0,>, 1,0,>, 0,0,>
14	39	11,2,>, 'expand', 1,4,>, 12,3,v, 0,3,v, 11,3,>, 10,2,>, 1,2,v, 0,2,v, 11,4,>, 10,4,>, 9,4,>, 8,4,>, 7,4,>, 6,4,>, 5,4,>, 4,4,>, 1,1,>, 12,0,v, 11,1,>, 10,1,>, 9,1,>, 8,1,>, 7,1,>, 6,1,>, 5,1,>, 4,1,>, 10,0,v, 9,0,v, 8,0,v, 7,0,v, 6,0,v, 5,0,v, 4,0,v, 2,0,v, 0,0,v, 11,0,>, 3,0,v, 1,0,>
15	21	12,2,>, 10,1,v, 8,1,v, 7,1,v, 6,1,v, 4,1,v, 1,1,>, 0,1,v, 13,1,>, 12,1,>, 11,1,>, 9,1,>, 5,1,>, 3,1,>, 6,0,>, 5,0,>, 4,0,>, 3,0,>, 2,0,>, 1,0,>, 0,0,>
16	45	13,2,>, 'expand', 1,4,>, 14,3,v, 0,3,v, 13,3,>, 12,2,>, 1,2,v, 0,2,v, 13,4,>, 12,4,>, 11,4,>, 10,4,>, 9,4,>, 8,4,>, 7,4,>, 6,4,>, 5,4,>, 4,4,>, 1,1,>, 14,0,v, 13,1,>, 12,1,>, 11,1,>, 10,1,>, 9,1,>, 8,1,>, 7,1,>, 6,1,>, 5,1,>, 4,1,>, 12,0,v, 11,0,v, 10,0,v, 9,0,v, 8,0,v, 7,0,v, 6,0,v, 5,0,v, 4,0,v, 2,0,v, 0,0,v, 13,0,>, 3,0,v, 1,0,>
17	24	14,2,>, 12,1,v, 10,1,v, 8,1,v, 6,1,v, 4,1,v, 1,1,>, 0,1,v, 15,1,>, 14,1,>, 13,1,>, 11,1,>, 9,1,>, 7,1,>, 5,1,>, 3,1,>, 7,0,>, 6,0,>, 5,0,>, 4,0,>, 3,0,>, 2,0,>, 1,0,>, 0,0,>
18	51	15,2,>, 'expand', 1,4,>, 16,3,v, 0,3,v, 15,3,>, 14,2,>, 1,2,v, 0,2,v, 15,4,>, 14,4,>, 13,4,>, 12,4,>, 11,4,>, 10,4,>, 9,4,>, 8,4,>, 7,4,>, 6,4,>, 5,4,>, 4,4,>, 1,1,>, 16,0,v, 15,1,>, 14,1,>, 13,1,>, 12,1,>, 11,1,>, 10,1,>, 9,1,>, 8,1,>, 7,1,>, 6,1,>, 5,1,>, 4,1,>, 14,0,v, 13,0,v, 12,0,v, 11,0,v, 10,0,v, 9,0,v, 8,0,v, 7,0,v, 6,0,v, 5,0,v, 4,0,v, 2,0,v, 0,0,v, 15,0,>, 3,0,v, 1,0,>
19	27	16,2,>, 14,1,v, 12,1,v, 10,1,v, 9,1,v, 8,1,v, 6,1,v, 4,1,v, 1,1,>, 0,1,v, 17,1,>, 16,1,>, 15,1,>, 13,1,>, 11,1,>, 7,1,>, 5,1,>, 3,1,>, 8,0,>, 7,0,>, 6,0,>, 5,0,>, 4,0,>, 3,0,>, 2,0,>, 1,0,>, 0,0,>
20	57	17,2,>, 'expand', 1,4,>, 18,3,v, 0,3,v, 17,3,>, 16,2,>, 1,2,v, 0,2,v, 17,4,>, 16,4,>, 15,4,>, 14,4,>, 13,4,>, 12,4,>, 11,4,>, 10,4,>, 9,4,>, 8,4,>, 7,4,>, 6,4,>, 5,4,>, 4,4,>, 1,1,>, 18,0,v, 17,1,>, 16,1,>, 15,1,>, 14,1,>, 13,1,>, 12,1,>, 11,1,>, 10,1,>, 9,1,>, 8,1,>, 7,1,>, 6,1,>, 5,1,>, 4,1,>, 16,0,v, 15,0,v, 14,0,v, 13,0,v, 12,0,v, 11,0,v, 10,0,v, 9,0,v, 8,0,v, 7,0,v, 6,0,v, 5,0,v, 4,0,v, 2,0,v, 0,0,v, 17,0,>, 3,0,v, 1,0,>
21	30	18,2,>, 16,1,v, 14,1,v, 12,1,v, 10,1,v, 8,1,v, 6,1,v, 4,1,v, 1,1,>, 0,1,v, 19,1,>, 18,1,>, 17,1,>, 15,1,>, 13,1,>, 11,1,>, 9,1,>, 7,1,>, 5,1,>, 3,1,>, 9,0,>, 8,0,>, 7,0,>, 6,0,>, 5,0,>, 4,0,>, 3,0,>, 2,0,>, 1,0,>, 0,0,>
22	>=61	??
23	33	20,2,>, 18,1,v, 16,1,v, 14,1,v, 12,1,v, 11,1,v, 10,1,v, 8,1,v, 6,1,v, 4,1,v, 1,1,>, 0,1,v, 21,1,>, 20,1,>, 19,1,>, 17,1,>, 15,1,>, 13,1,>, 9,1,>, 7,1,>, 5,1,>, 3,1,>, 10,0,>, 9,0,>, 8,0,>, 7,0,>, 6,0,>, 5,0,>, 4,0,>, 3,0,>, 2,0,>, 1,0,>, 0,0,>
```

Have fun running it with more time and RAM.

Sadly, the sequence "3,3,6,6,9,9,15,12,26,15,33,18,39,21,45,24,51,27,57,30,>=61" is [not known](https://oeis.org/search?q=3,3,6,6,9,9,15).
What comes next?  Who knows? :D

Since odd degrees are trivial, we might instead want to look at even degrees:
But the sequence "3,6,9,15,26,33,39,45,51,57,>=61" is also [not known](https://oeis.org/search?q=3,6,9,15,26,33),
neither is the [difference sequence](https://oeis.org/search?q=3,3,6,11,7).

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
