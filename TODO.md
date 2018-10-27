* See if we can get the info for send_client_info programmatically from Cargo.toml
  or the code
  * Check out a build.rs

* Healthcheck:
  * Add version info (see languageclient_neovim for how to do it properly?) Remember to 
    query the binary, in case there' ambiguity
  * List all available binaries, not only the one used 

* Remove failure from the lib, implement own error handling

* Try out crossbeam's channel rather than the one from std

* Check out https://github.com/kernelmachine/cargo-profiler

* Setup fuzzing?
  * Checkc https://users.rust-lang.org/t/announcing-afl-rs-0-2-bindings-for-american-fuzzy-lop/13981

* Work through https://rust-lang-nursery.github.io/api-guidelines/

* Check out for docs:
  * https://github.com/Geal/cargo-external-doc
  * https://github.com/vitiral/artifact

* Improve example usage in src/folds.rs

* Write more tests for conditionals in src/card/line.rs

### Things to keep in mind when releasing

* Check out https://www.makeareadme.com/#usage
* Update CHANGELOG.md

### Performance stuff tried and discarded because the benchmark(s) didn't improve

* Turn off UTF8 validation in rmpv
* Implement the foldlist as a Vec instead of a BTreeMap
* Changing CondResult::Number to take a u32 instead of usize was a clear
  performance regression. Going to u16 was even worse.
  * Similarly changing Int to usize/u32/u16 from u8
  * Changin RelChar's char to u8 doesn't help (might need anyways later when switching
    from AsRef<str> to AsRef<[u8]>
  * Changing the Ranges from usize to u8 had a minor impact, but it's semantically
    more correct anyways

### Performance ideas that might not be neccessary
* If performance isn't good, see 
  https://www.reddit.com/r/rust/comments/7h4q0i/can_this_function_be_improved_performancewise/dqoolbm/

* Make an option to set the binary path without checking for it for faster startup

* skip_card doesn't need to allocate a vec most of the time!
  * Maybe revert that? Doesn't really help a lot, though a tad indeed

* Before parsing the vec, maybe sort it?

### Old stuff, not sure about this
* The new test is racy. Things to do about it
  * Maybe issue a synchronous rpcrequest to get the current buffer?
  * Maybe don't request the current buffer, but luaeval a function that returns
    the buffer for that channel (how? not sure -> nvim_get_api_info)
  * Maybe add a new event to the plugin that waits for the buffer number from
    rpcnotify, and oonly after receiving that start the "real" event loop
* Maybe open preview window for the foldtexts/stderr when requested?
  Try the following by @chemzqm
    pclose
    keepalt new +setlocal\ previewwindow|setlocal\ buftype=nofile|setlocal\ noswapfile|setlocal\ wrap [Document]
    setl bufhidden=wipe
    setl nobuflisted
    setl nospell
    setl filetype=markdown
    let lines = split(a:info, "\n")
    call append(0, lines)
    exe "normal z" . len(lines) . "\<cr>"
    exe "normal gg"
    wincmd p
