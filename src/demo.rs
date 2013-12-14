#!/usr/bin/env rustx
#[feature(non_ascii_idents)];
extern mod extra;
use std::os;

#[main]
fn say_hi () {
   println("Hello!!");
   os::set_exit_status(0);
}

#[test]
fn addition_works () {
   assert! (2 + 2 == 4);
}

#[bench]
fn addition_benchmarked (b: &mut extra::test::BenchHarness) {
   let mut sum = 0;
   b.iter(|| sum += 1)
}
