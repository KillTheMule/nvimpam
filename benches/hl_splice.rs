extern crate nvimpam_lib;

#[macro_use]
extern crate criterion;
extern crate neovim_lib;

use std::fs;

use criterion::{black_box, Criterion};

use neovim_lib::{Value, neovim_api::Buffer};

use nvimpam_lib::{
  bufdata::BufData,
  linenr::LineNr,
};

macro_rules! hl_bench {
  ($fn: ident; lines: ($start: expr, $end: expr);
   spliceto: ($sstart: expr, $ssend: expr, $added: expr)) => {
    fn $fn(c: &mut Criterion) {
      use nvimpam_lib::bufdata::highlights::Highlights;
      use neovim_lib::{Value, neovim_api::Buffer};

      c.bench_function(stringify!($fn), move |b| {
        let buf = Buffer::new(Value::from(0_usize));
        let origlines = fs::read("files/example.pc").expect("1");
        let mut bufdata = BufData::new(&buf);
        bufdata.parse_slice(&origlines).expect("2");

        // example.pc has 20586 lines, so 20587 is the last valid linenumber
        // so 20586 is the last valid line index
        assert!($end < 20587);
        assert!($ssend < 20587);

        let v: Vec<_> = bufdata
          .highlights
          .iter()
          .filter(|((l, _, _), _)| {
            LineNr::from_usize($start) <= *l && *l < LineNr::from_usize($end)
          })
          .map(|((l, s, e), h)| ((*l, *s, *e), *h))
          .collect();

        b.iter(move || {
          let newhls: Highlights = Highlights(v.clone());
          let range =
            bufdata.highlights.splice(newhls, $sstart.into(), $ssend.into(), $added);

          let _calls = black_box(bufdata.highlight_region_calls(
            range.clone(),
            range.start.into(),
            range.end.into(),
          ));
        })
      });
    }
  };
}

fn bench_bufdata_create(c: &mut Criterion) {
  c.bench_function("bench_bufdata_create", move |b| {
    let origlines = fs::read("files/example.pc").expect("1");
    let buf = Buffer::new(Value::from(0_usize));
    let mut bufdata = BufData::new(&buf);

    b.iter(|| {
      bufdata.clear();
      bufdata.parse_slice(&origlines).expect("2");
    })
  });
}

fn bench_bufdata_readonly(c: &mut Criterion) {
  c.bench_function("bench_bufdata_readonly", move |b| {
    let origlines = fs::read("files/example.pc").expect("1");
    let buf = Buffer::new(Value::from(0_usize));
    let mut bufdata = BufData::new(&buf);
    bufdata.parse_slice(&origlines).expect("2");

    b.iter(|| {
      let _calls = black_box(bufdata.highlight_region_calls(
        1000..10000,
        1000.into(),
        10000.into(),
      ));
    })
  });
}

hl_bench!(
  bench_bufdata_change_line_start;
  lines: (28, 29);
  spliceto: (28, 29, 0)
  );

hl_bench!(
  bench_bufdata_change_line_end;
  lines: (20500, 20501);
  spliceto: (20500, 20501, 0)
  );

hl_bench!(
  bench_bufdata_add_line_start;
  lines: (28, 30);
  spliceto: (28, 29, 1)
  );

hl_bench!(
  bench_bufdata_add_line_end;
  lines: (20500, 20502);
  spliceto: (20500, 20501, 1)
  );

hl_bench!(
  bench_bufdata_delete_line_start;
  lines: (1, 1);
  spliceto: (28, 29, -1_isize)
  );

hl_bench!(
  bench_bufdata_delete_line_end;
  lines: (1, 1);
  spliceto: (20500, 20501, -1_isize)
  );

hl_bench!(
  bench_bufdata_paste_after;
  lines: (28, 35);
  spliceto: (20586, 20586, 7)
  );

hl_bench!(
  bench_bufdata_paste_before;
  lines: (28, 35);
  spliceto: (0, 0, 7)
  );

criterion_group!(
  name = hl_splice;
  config = Criterion::default().sample_size(10).without_plots();
  targets = bench_bufdata_create,
            bench_bufdata_change_line_start,
            bench_bufdata_change_line_end,
            bench_bufdata_add_line_start,
            bench_bufdata_add_line_end,
            bench_bufdata_delete_line_start,
            bench_bufdata_delete_line_end,
            bench_bufdata_paste_after,
            bench_bufdata_paste_before,
            bench_bufdata_readonly,
);
criterion_main!(hl_splice);
