extern crate nvimpam_lib;

#[macro_use]
extern crate criterion;
use criterion::{black_box, Criterion};

use nvimpam_lib::{bufdata::BufData, card::keyword::Keywords, lines::Lines};

fn bench_bufdata_create(c: &mut Criterion) {
  c.bench_function("bufdata_create", move |b| {
    let origlines = Lines::read_file("files/example.pc").expect("1");
    let lines = Lines::from_slice(&origlines);
    let keywords = Keywords::from_lines(&lines);
    let mut bufdata = BufData::new();

    b.iter(|| {
      bufdata.recreate_all(&keywords, &lines).expect("2");
    })
  });
}

fn bench_bufdata_change_line(c: &mut Criterion) {
  c.bench_function("bufdata_change_line", move |b| {
    let origlines = Lines::read_file("files/example.pc").expect("1");
    let lines = Lines::from_slice(&origlines);
    let keywords = Keywords::from_lines(&lines);
    let mut bufdata = BufData::new();
    bufdata.recreate_all(&keywords, &lines).expect("2");

    b.iter(|| {
      let mut tmp_bufdata = black_box(BufData::new());
      tmp_bufdata
        .recreate_all(&keywords[28..29], &lines[28..29])
        .expect("3");
      bufdata.splice(tmp_bufdata, 28, 29, 0);
    })
  });
}

fn bench_bufdata_add_line(c: &mut Criterion) {
  c.bench_function("bufdata_add_line", move |b| {
    let origlines = Lines::read_file("files/example.pc").expect("1");
    let lines = Lines::from_slice(&origlines);
    let keywords = Keywords::from_lines(&lines);
    let mut bufdata = BufData::new();
    bufdata.recreate_all(&keywords, &lines).expect("2");

    b.iter(|| {
      let mut tmp_bufdata = black_box(BufData::new());
      tmp_bufdata
        .recreate_all(&keywords[28..30], &lines[28..30])
        .expect("3");
      bufdata.splice(tmp_bufdata, 28, 29, 1);
    })
  });
}

fn bench_bufdata_delete_line(c: &mut Criterion) {
  c.bench_function("bufdata_delete_line", move |b| {
    let origlines = Lines::read_file("files/example.pc").expect("1");
    let lines = Lines::from_slice(&origlines);
    let keywords = Keywords::from_lines(&lines);
    let mut bufdata = BufData::new();
    bufdata.recreate_all(&keywords, &lines).expect("2");

    b.iter(|| {
      let tmp_bufdata = black_box(BufData::new());
      bufdata.splice(tmp_bufdata, 28, 29, -1);
    })
  });
}

fn bench_bufdata_paste_after(c: &mut Criterion) {
  c.bench_function("bufdata_paste_after", move |b| {
    let origlines = Lines::read_file("files/example.pc").expect("1");
    let lines = Lines::from_slice(&origlines);
    let keywords = Keywords::from_lines(&lines);
    let mut bufdata = BufData::new();
    bufdata.recreate_all(&keywords, &lines).expect("2");

    let last = lines.len() - 1;

    b.iter(|| {
      let mut tmp_bufdata = black_box(BufData::new());
      tmp_bufdata
        .recreate_all(&keywords[28..35], &lines[28..35])
        .expect("3");
      bufdata.splice(tmp_bufdata, last, last, 7);
    })
  });
}

fn bench_bufdata_paste_before(c: &mut Criterion) {
  c.bench_function("bufdata_paste_before", move |b| {
    let origlines = Lines::read_file("files/example.pc").expect("1");
    let lines = Lines::from_slice(&origlines);
    let keywords = Keywords::from_lines(&lines);
    let mut bufdata = BufData::new();
    bufdata.recreate_all(&keywords, &lines).expect("2");

    b.iter(|| {
      let mut tmp_bufdata = black_box(BufData::new());
      tmp_bufdata
        .recreate_all(&keywords[28..35], &lines[28..35])
        .expect("3");
      bufdata.splice(tmp_bufdata, 0, 0, 7);
    })
  });
}
criterion_group!(
  name = hl_splice;
  config = Criterion::default().sample_size(10).without_plots();
  targets = bench_bufdata_change_line,
            bench_bufdata_create,
            bench_bufdata_add_line,
            bench_bufdata_delete_line,
            bench_bufdata_paste_after,
            bench_bufdata_paste_before
);
criterion_main!(hl_splice);
