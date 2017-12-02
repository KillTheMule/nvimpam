#![feature(test)]
extern crate test;
extern crate nvimpam_lib;

use self::test::Bencher;

use nvimpam_lib::cards::Card;

#[bench]
fn bench_parse2folddata(b: &mut Bencher) {
  use std::fs::File;
  use std::io::{self, BufRead};

  let file = File::open("files/example.pc").unwrap();
  let v: Vec<String> = io::BufReader::new(file)
    .lines()
    .map(|l| l.unwrap())
    .collect();

  b.iter(|| {
    let r = test::black_box(&v);
    let compacted = Card::contract_card_data_direct(r);
  })
}
