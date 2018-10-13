extern crate nvimpam_lib;

#[macro_use]
extern crate criterion;
use criterion::Criterion;

use nvimpam_lib::card::ges::GesType;
use nvimpam_lib::card::keyword::Keyword;
use nvimpam_lib::folds::FoldList;
use nvimpam_lib::nocommentiter::CommentLess;

fn bench_parse2folddata(c: &mut Criterion) {
  c.bench_function("card_parse2folddata", |b| {
    use std::fs::File;
    use std::io::{self, BufRead};

    let file = File::open("files/example.pc").unwrap();
    let v: Vec<String> = io::BufReader::new(file)
      .lines()
      .map(|l| l.unwrap())
      .collect();
    // let v = include!("../files/example.rs");

    let mut f = FoldList::new();
    b.iter(|| {
      let r = &v;
      f.clear();
      let _compacted = f.add_folds(r);
    });
  });
}

fn bench_parse_str(c: &mut Criterion) {
  c.bench_function("card_parse_str", |b| {
    use std::fs::File;
    use std::io::{self, BufRead};

    let file = File::open("files/example.pc").unwrap();
    let v: Vec<String> = io::BufReader::new(file)
      .lines()
      .map(|l| l.unwrap())
      .collect();

    b.iter(|| {
      let r = &v;
      let _parsed: Vec<Option<Keyword>> =
        r.iter().map(|s| Keyword::parse(s)).collect();
    });
  });
}

const GES: [&str; 9] = [
  "        PART 1234",
  "        OGRP 'hausbau'",
  "        END",
  "        DELGRP>NOD 'nix'",
  "        MOD 10234",
  "        NOD 1 23 093402 82",
  "        END_MOD",
  "        DELELE 12",
  "        END",
];

fn bench_skip_ges(c: &mut Criterion) {
  c.bench_function("card_skip_ges", |b| {
    let g = GesType::GesNode;
    b.iter(|| {
      let mut li = GES.iter().enumerate().remove_comments();
      let mut tmp = li.next().unwrap();
      let mut _a = li.skip_ges(g, &tmp);
      tmp = li.next().unwrap();
      _a = li.skip_ges(g, &tmp);
    });
  });
}

criterion_group!(
  name = card;
  config = Criterion::default();
  targets = bench_parse2folddata, bench_parse_str, bench_skip_ges
);
criterion_main!(card);
