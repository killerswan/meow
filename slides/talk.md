% Testing in Rust
% Kevin Cantu @killerswan
% February 25, 2014




-------
## "Thou shalt not test the Lord thy God!"
-- The Bible




# Overview

1. The *scientific method*.

2. Rust's fundamental toolkit.

3. Something useful I'm playing with: **testloop**.

4. Future directions.




# What is the scientific method?




# What is the scientific method?

1. We cannot see everything.
2. Good claims make **testable predictions**.




# What is the scientific method?

1. We cannot see everything.
2. Good claims make **testable predictions**.

<The culture around this is essential.>




# But what if I can *prove* it mathematically?

toxoplasma gondii: ~20 MBP

--> mind control of higher-order mammals




--------
baboons in luggage GIF: FIXME




# Cheap negative proofs:

1. Failed type checking.
2. Failing tests.

(Bug reports and exploits are expensive.)


(meredith patterson likes to say exploits are proofs, too)




--------
goto fail;
goto fail;  // FIXME




# Rust's toolbox for testing




# Hello world
```rust
fn main() {}
```

```sh
$ rustc        main.rs -o main
$ ./main
```


# Hello world of tests
```rust
#[test]
fn t1() {}
```

```sh
$ rustc --test main.rs -o test
$ ./test
```




# Sharding
Machine 1:
```sh
$ ./test --......FIXME.... SHARDING SYNTAX
```

Machine 2:
```sh
$ ./test --......FIXME.... SHARDING SYNTAX
```





# Benchmarks
```rust
#[bench]
fn t2() {}
```

```sh
$ rustc --test main.rs -o test
$ ./test --bench
```




# Metrics

```sh
$ ./test --bench --save FIXME
$ ./test --bench --metrics FIXME
```






# Rust.CI
.travis.yml:
```yml

```


# Rust.CI
Signup page
FIXME



# Rust.CI
FAILURE EMAIL FIXME




# Experiment: testloop
```sh
$ rustpkg testloop -h
Usage: rustpkg testloop [--bench]
```

Run while editing your code for instant feedback.



# Things we really need more of

* QuickCheck
* development environments (IDEs, REPLs, novel things)
* integration




# Sources

# References

# More

Kevin Cantu (@killerswan) <<me@kevincantu.org>>

[https://github.com/killerswan/meow](https://github.com/killerswan/meow)

