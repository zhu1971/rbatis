#![feature(test)]
extern crate test;

use test::Bencher;
use rbs::value::util::extract_number;

//test bench_extract_number ... bench:           3 ns/iter (+/- 0)
#[bench]
fn bench_extract_number(b: &mut Bencher) {
    let v ="1.111";
    b.iter(|| {
       extract_number(v);
    });
}