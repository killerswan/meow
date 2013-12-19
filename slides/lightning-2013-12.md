% Testing your Rust
% Kevin Cantu (@killerswan)
% December 17, 2013

-------

## "If we don't try, we will not know how our luck falls."
-- Saga of King Hrolf Kraki

## "Modification is undesirable, but modifiability is paramount."
-- Paul Phillips (@extempore2) [at PNW Scala 2013](http://www.youtube.com/watch?v=TS1lpKBMkgg)

------
# Meow
[https://github.com/killerswan/meow](https://github.com/killerswan/meow)

![window_cat.jpg](static/window_cat.jpg)

# Overview
Learn about Rust's tests and benchmarks, then explore *sharding* and *metrics*.

Do continuous integration with **Rust.CI**.

Try **testloop**, my tool for TDD.

# Flags to `rustc`
```sh
rustc --bin  main.rs -o main
rustc --test main.rs -o main_tests
```

# Symbols when compiled (--bin)
* tests are skipped
```
EXCERPT
```

# my makefile / substitute rustpkg

# tying it all together
```sh
$ rustpkg testloop -h
Usage: rustpkg testloop [--bench] [--opt-level [0-3]]
```






