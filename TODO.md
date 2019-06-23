* Test that the initial update when requesting it really works

* Use proper namespace for highlights, right now we're just using '5'

* Push most of the handling for HighlightRegion in event.rs into
  bufdata/mod.rs, so the event handling itself stays simple

* Improve NvimPamCommentCard, needs though about GES and stuff

* Make a "card" text object

* Let the events from the handler return LineNr or a future ColNr and handle all assertions and
  conversions there, to clean up event.rs

* Define the CursorMoved(I) aucmds only after we've loaded the plugin, and have
  it update the first cellhint by itself. Otherwise we're blocking while loading the file.

* Returning an Option from first_after/last_before made several parts ugly. Look what we can do
  about that 

* Folds don't see non-comment lines without a keyword. If there's stuff before
  the header, it might be included in the fold. Maybe just ignore that, since it's 
  syntactically invalid anyways

* Send the bufnr as a notification directly after jobstart to avoid races
  * Or maybe just from the commandline?

* Improve logging: Lua code should write to NVIMPAM_LOG_FILE (maybe even respect log llevel?), only the binaries stderr should really go to NVIMPAM_STDERR

* Check out using lookup tables for performance improvements. Ref https://github.com/lynaghk/question-rust-inlining also see https://github.com/sfackler/rust-phf

* Startup is racy, as can be seen when the insertions test is ran without a sleep call. Maybe only register commands after startup? Maybe defer calls in lua when requested during startup?

* Save lines by immutable ID and provide a map ID -> linenr somewhere? That
  would make updating the linenumbers somewhat easier.

* Write a size hint (exact!) for the HlIter

* Employ OptionSet aucmds to set the colors (can we maybe do it by using the color schemes colors?)

* See if we can get the info for send_client_info programmatically from Cargo.toml
  or the code
  * Check out a build.rs

* Healthcheck:
  * Add version info (see languageclient_neovim for how to do it properly?) Remember to 
    query the binary, in case there' ambiguity
  * List all available binaries, not only the one used 
  * Also, really show the one used, not the one discovered when running the healthcheck

* Remove failure from the lib, implement own error handling

* Try out crossbeam's channel rather than the one from std

* Check out https://github.com/kernelmachine/cargo-profiler

* Setup fuzzing?
  * Checkc https://users.rust-lang.org/t/announcing-afl-rs-0-2-bindings-for-american-fuzzy-lop/13981

* Work through https://rust-lang-nursery.github.io/api-guidelines/

* Check out for docs:
  * https://github.com/Geal/cargo-external-doc
  * https://github.com/vitiral/artifact

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
