* The new test is racy. Things to do about it
  * Maybe issue a synchronous rpcrequest to get the current buffer?
  * Maybe don't request the current buffer, but luaeval a function that returns
    the buffer for that channel (how? not sure -> nvim_get_api_info)
  * Maybe add a new event to the plugin that waits for the buffer number from
    rpcnotify, and oonly after receiving that start the "real" event loop

* Implement nvim_set_client_info

* Add level 2 folds

* Remove failure from the lib, implement own error handling

* Work on skip_card if we can't reduce the amount of repetition 

* Check out https://github.com/kernelmachine/cargo-profiler

* Check out http://troubles.md/posts/rustfest-2018-workshop/ for using cachgegrind for
  performance analysis

* skip_card doesn't need to allocate a vec most of the time!
  * Maybe revert that? Doesn't really help a lot, though a tad indeed

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

* If performance isn't good, see 
  https://www.reddit.com/r/rust/comments/7h4q0i/can_this_function_be_improved_performancewise/dqoolbm/

* Before parsing the vec, maybe sort it?

* Check out https://www.makeareadme.com/#usage

* Unify skip functions wrt usage of curkw, curidx vs. line, lineidx
