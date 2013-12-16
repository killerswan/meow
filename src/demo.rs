#[feature(non_ascii_idents)];
#[allow(dead_code)];

extern mod extra;
use std::os;

#[main]
fn say_hi () {
   let _args = os::args();
   println("Hello!!");
   os::set_exit_status(0);
}

#[test]
fn addition_works () {
   assert! (2 + 2 == 4);
}

fn something() {
   std::io::timer::sleep(5);
}

#[bench]
fn addition_benchmarked (b: &mut extra::test::BenchHarness) {
   b.iter(|| {
      something();
   })
}
