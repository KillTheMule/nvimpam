#![feature(test)]

extern crate nvimpam_lib;

extern crate test;
use self::test::Bencher;
use std::alloc::System;
#[global_allocator]
static GLOBAL: System = System;

use nvimpam_lib::{
  card::{
    ges::GesType,
    keyword::{Keyword, Keywords},
  },
  folds::FoldList,
  lines::{Lines, ParsedLine},
  nocommentiter::CommentLess,
};

#[bench]
fn bench_parse2folddata(b: &mut Bencher) {
  use std::{
    fs::File,
    io::{self, BufRead},
  };

  let file = File::open("files/example.pc").unwrap();
  let v: Vec<String> = io::BufReader::new(file)
    .lines()
    .map(|l| l.unwrap())
    .collect();
  let w: Vec<&str> = v.iter().map(|l| l.as_ref()).collect();

  let mut f = FoldList::new();
  b.iter(|| {
    let lines = Lines::from_strs(&w[..]);
    let keywords = Keywords::from_lines(&lines);
    f.clear();
    let _compacted = f.add_folds(&keywords, &lines);
  });
}

#[bench]
fn bench_parse_str(b: &mut Bencher) {
  use std::{
    fs::File,
    io::{self, BufRead},
  };

  let file = File::open("files/example.pc").unwrap();
  let v: Vec<String> = io::BufReader::new(file)
    .lines()
    .map(|l| l.unwrap())
    .collect();

  b.iter(|| {
    let r = &v;
    let _parsed: Vec<Option<Keyword>> =
      r.iter().map(|s| Keyword::parse(s.as_ref())).collect();
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

#[bench]
fn bench_skip_ges(b: &mut Bencher) {
  let g = GesType::GesNode;
  b.iter(|| {
    let lines = Lines::from_strs(&GES);
    let keywords: Keywords = Keywords::from_lines(&lines);
    let mut li = keywords
      .iter()
      .zip(lines.iter())
      .enumerate()
      .map(ParsedLine::from)
      .remove_comments();
    let mut tmp = li.next().unwrap();
    let mut _a = li.skip_ges(g, &tmp);
    tmp = li.next().unwrap();
    _a = li.skip_ges(g, &tmp);
  });
}
