* skip_card doesn't need to allocate a vec most of the time!

* Do we need the channel between the handler and the event loop? How about just having the handler call what's neccessary?

* Check out https://github.com/sunng87/cargo-release

* Add benchmarks
  * Maybe https://docs.rs/bencher/0.1.4/bencher/?
  * Need integration benchmarks, so make something cust0m in viml?
    * No, have the test setup nvim as a child
  * Check out criterion.rs

* Setup fuzzing?
  * Checkc https://users.rust-lang.org/t/announcing-afl-rs-0-2-bindings-for-american-fuzzy-lop/13981

* Work through https://rust-lang-nursery.github.io/api-guidelines/

* Check out for docs:
  * https://github.com/Geal/cargo-external-doc
  * https://github.com/vitiral/artifact

* Mention logging in the doc

* Write vim doc

* If performance isn't good, see 
  https://www.reddit.com/r/rust/comments/7h4q0i/can_this_function_be_improved_performancewise/dqoolbm/

* Before parsing the vec, maybe sort it?

* Both the Line enum variant and the trait are called ges. Bad?
* Check out https://www.makeareadme.com/#usage

* Unify skip functions wrt usage of curkw, curidx vs. line, lineidx
* Write correct doc comments in lines.rs and folds.rs
