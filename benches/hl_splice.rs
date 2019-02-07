extern crate nvimpam_lib;

#[macro_use]
extern crate criterion;
extern crate neovim_lib;

use criterion::{black_box, Criterion};

use neovim_lib::Value;

use nvimpam_lib::{
  bufdata::{highlights::HighlightGroup as Hl, BufData},
  lines::Lines,
};

fn fake_highlight_region<'a, 'b, 'c, T>(
  iter: T,
  firstline: u64,
  lastline: u64,
) -> Vec<Value>
where
  T: Iterator<Item = (&'b (u64, u8, u8), &'b Hl)>,
{
  let mut calls: Vec<Value> = vec![];

  calls.push(
    vec![
      Value::from("nvim_buf_clear_highlight".to_string()),
      vec![
        Value::from(1),
        Value::from(5),
        Value::from(firstline),
        Value::from(lastline),
      ]
      .into(),
    ]
    .into(),
  );

  for ((l, s, e), t) in iter {
    let st: &'static str = (*t).into();
    calls.push(
      vec![
        Value::from("nvim_buf_add_highlight".to_string()),
        vec![
          Value::from(1),
          Value::from(5),
          Value::from(st.to_string()),
          Value::from(*l),
          Value::from(u64::from(*s)),
          Value::from(u64::from(*e)),
        ]
        .into(),
      ]
      .into(),
    );
  }
  calls
}

macro_rules! hl_bench {
  ($fn: ident; lines: ($start: expr, $end: expr);
   spliceto: ($sstart: expr, $ssend: expr, $added: expr)) => {
    fn $fn(c: &mut Criterion) {
      use nvimpam_lib::bufdata::highlights::Highlights;

      c.bench_function(stringify!($fn), move |b| {
        let origlines = Lines::read_file("files/example.pc").expect("1");
        let mut bufdata = BufData::new();
        bufdata.parse_slice(&origlines).expect("2");

        // example.pc has 20586 lines, so 20587 is the last valid linenumber
        // so 20586 is the last valid line index
        assert!($end < 20587);
        assert!($ssend < 20587);

        let v: Vec<_> = bufdata
          .highlights
          .iter()
          .filter(|((l, _, _), _)| $start <= *l && *l < $end)
          .map(|((l, s, e), h)| ((*l, *s, *e), *h))
          .collect();

        b.iter(move || {
          let newhls: Highlights = Highlights(v.clone());
          let (start, end) =
            bufdata.highlights.splice(newhls, $sstart, $ssend, $added);

          let _calls = black_box(fake_highlight_region(
            bufdata.highlights.indexrange(start, end),
            start as u64,
            end as u64,
          ));
        })
      });
    }
  };
}

fn bench_bufdata_create(c: &mut Criterion) {
  c.bench_function("bench_bufdata_create", move |b| {
    let origlines = Lines::read_file("files/example.pc").expect("1");
    let mut bufdata = BufData::new();

    b.iter(|| {
      bufdata.parse_slice(&origlines).expect("2");
    })
  });
}

fn bench_bufdata_readonly(c: &mut Criterion) {
  c.bench_function("bench_bufdata_readonly", move |b| {
    let origlines = Lines::read_file("files/example.pc").expect("1");
    let mut bufdata = BufData::new();
    bufdata.parse_slice(&origlines).expect("2");

    b.iter(|| {
      let _calls = black_box(fake_highlight_region(
        bufdata.highlights.linerange(1000, 10000),
        1000,
        10000,
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
  spliceto: (28, 29, -1i64)
  );

hl_bench!(
  bench_bufdata_delete_line_end;
  lines: (1, 1);
  spliceto: (20500, 20501, -1i64)
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
