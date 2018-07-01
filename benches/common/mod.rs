extern crate criterion;
use criterion::Criterion;

pub fn conf() -> Criterion {
  use std::env;
  use std::time::Duration;

  if env::var_os("APPVEYOR").is_some() || env::var_os("TRAVIS").is_some() {
    Criterion::default()
      .sample_size(2)
      .warm_up_time(Duration::from_nanos(1))
      .measurement_time(Duration::from_nanos(1))
      .nresamples(1)
  } else {
    Criterion::default()
  }
}
