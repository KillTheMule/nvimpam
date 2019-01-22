extern crate nvimpam_lib;

#[macro_use]
extern crate criterion;
use criterion::{Criterion, black_box};

use nvimpam_lib::{bufdata::BufData, card::keyword::Keywords, lines::Lines};

fn bench_bufdata_create(c: &mut Criterion) {
  c.bench_function("bufdata_create", move |b| {
    let origlines = Lines::read_file("files/example.pc").expect("1");
    let lines = Lines::from_slice(&origlines);
    let keywords = Keywords::from_lines(&lines);
    let mut bufdata = BufData::new();
    bufdata.recreate_all(&keywords, &lines).expect("2");


    b.iter(|| {
      let mut tmp_bufdata = black_box(BufData::new());
      tmp_bufdata.recreate_all(&keywords[28..29], &lines[28..29]).expect("3");
      bufdata.splice(tmp_bufdata, 28, 29, 0);
    })
  });
}

criterion_group!(name = hl_splice; config = Criterion::default().sample_size(10).without_plots(); targets = bench_bufdata_create);
criterion_main!(hl_splice);
