# nvimpam

The beginning of a neovim rpc plugin for pamcrash files. Based on
[neovim-scorched-earth](https://github.com/boxofrox/neovim-scorched-earth).

Uses [daa84/neovim-lib](https://github.com/daa84/neovim-lib). Uses
https://github.com/neovim/neovim/pull/5269, so building a patched neovim is
necessary (patched sources included).

## Running tests

In the neovim folder, run

```sh
TEST_FILE=../test/nvimpam_spec.lua make functionaltest
```

This will use the test infrastructure of neovim to run a functional test.

## License

Dual-Licensed under Apache or MIT at your leisure, see the LICENSE-\* files.

## CoC

Wherever applicable, this project follows the [rust code of
conduct](https://www.rust-lang.org/en-US/conduct.html).
