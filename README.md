# nvimpam  
[![(Travis status)](https://travis-ci.org/KillTheMule/nvimpam.svg?branch=master)](https://travis-ci.org/KillTheMule/nvimpam)
[![(Appveyor status)](https://ci.appveyor.com/api/projects/status/mnmy5bpq895sklwy/branch/master?svg=true)](https://ci.appveyor.com/project/KillTheMule/nvimpam)
[![(Docs.rs)](https://docs.rs/nvimpam/badge.svg)](https://docs.rs/nvimpam/)
[![(Crates.io status)](https://meritbadge.herokuapp.com/nvimpam)](https://crates.io/crates/nvimpam)

The beginning of a neovim rpc plugin for pamcrash files. Right now, it does folding. Future dreams include syntax highlighting and status line hints. 

Based on [neovim-scorched-earth](https://github.com/boxofrox/neovim-scorched-earth). Uses [daa84/neovim-lib](https://github.com/daa84/neovim-lib). 

## Installation

First and foremost, you will need the nvimpam binary. For that, get a [rust](https://www.rust-lang.org/en-US/install.html) installation (the stable release is sufficient), and run `cargo install nvimpam` (you will need to have the installation directory in your PATH). If requested, I'd provide prebuilt binaries as well, just let me know through a github issue. 

To get the plugin files, either point your plugin manager to the github repository, or copy the following folders into your neovim config directory (see `:h xdg`): `ftdetect`, `ftplugin`, `lua`, `doc`.

## Usage

See `:h nvimpam` for usage hints.

## Contributing

I'd love contributions, comments, praise, criticism... You could open an [issue](https://github.com/KillTheMule/nvimpam/issues) or a [pull request](https://github.com/KillTheMule/nvimpam/pulls), or if you want a direct contact, meet me in the [neovim gitter channel](https://gitter.im/neovim/neovim). I also read the subreddits for [rust](https://www.reddit.com/r/rust/) and [neovim](https://www.reddit.com/r/neovim/), if that suits you better.

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

## License

Dual-Licensed under Apache or MIT at your leisure, see the LICENSE-\* files.

## CoC

Wherever applicable, this project follows the [rust code of
conduct](https://www.rust-lang.org/en-US/conduct.html).
