# nvimpam  ![Travis status](https://travis-ci.org/KillTheMule/nvimpam.svg?branch=master) ![Appveyor status](https://ci.appveyor.com/api/projects/status/mnmy5bpq895sklwy/branch/master?svg=true)

The beginning of a neovim rpc plugin for pamcrash files. Right now, it does some folding, badly. Future dreams include good folding, syntax highlighting and status line hints. 

Based on [neovim-scorched-earth](https://github.com/boxofrox/neovim-scorched-earth). Uses [daa84/neovim-lib](https://github.com/daa84/neovim-lib). Also uses https://github.com/neovim/neovim/pull/7917, so building a patched neovim is necessary (patched sources included).

## Running tests

Running

```sh
cargo test
```
in the main folder will run the tests in the rust code and documentation. There will not be many of those, most of the testing will be done through neovim functional tests (written in lua). For those, run

```sh
TEST_FILE=../test/nvimpam_spec.lua make functionaltest
```

in the `neovim` folder of this repository. This might take some time on th first run because it needs to compile neovim and its dependencies.

## Contributing

I'd love contributions, comments, praise, criticism... You could open an [issue](https://github.com/KillTheMule/nvimpam/issues) or a [pull request](https://github.com/KillTheMule/nvimpam/pulls), or if you want a direct contact, meet me in the [neovim gitter channel](https://gitter.im/neovim/neovim). I also read the subreddits for [rust](https://www.reddit.com/r/rust/) and [neovim](https://www.reddit.com/r/neovim/), if that suits you better.

## License

Dual-Licensed under Apache or MIT at your leisure, see the LICENSE-\* files.

## CoC

Wherever applicable, this project follows the [rust code of
conduct](https://www.rust-lang.org/en-US/conduct.html).
