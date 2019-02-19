extern crate nvimpam_lib;
extern crate neovim_lib;

#[macro_use]
extern crate criterion;

use std::fs;

use criterion::Criterion;

use neovim_lib::{Value, neovim_api::Buffer};

use nvimpam_lib::{
  bufdata::BufData,
  card::{
    ges::GesType,
    keyword::Keyword,
  },
  lines::{Lines, ParsedLine},
  linesiter::LinesIter,
  linenr::LineNr,
};

fn bench_parse2bufdata(c: &mut Criterion) {
  c.bench_function("card_parse2folddata", |b| {
    let origlines = fs::read("files/example.pc").expect("3.1");

    let buf = Buffer::new(Value::from(0_usize));
    let mut bufdata = BufData::new(&buf);
    b.iter(|| {
      bufdata.clear();
      bufdata.parse_slice(&origlines).expect("4");
    });
  });
}

fn bench_parse_str(c: &mut Criterion) {
  c.bench_function("card_parse_str", |b| {
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
    let mut lines = Lines::new();
    lines.parse_strs(&GES);

    b.iter(|| {
      let mut li: LinesIter<_> = lines.iter();
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
  targets = bench_parse2bufdata, bench_parse_str, bench_skip_ges
);
criterion_main!(card);
