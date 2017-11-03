# nvimpam

The beginning of a neovim rpc plugin for pamcrash files. Based on
[neovim-scorched-earth](https://github.com/boxofrox/neovim-scorched-earth).

Uses [daa84/neovim-lib](https://github.com/daa84/neovim-lib), a slightly
modified copy is included. Uses https://github.com/neovim/neovim/pull/5269, so
building a patched neovim is necessary (I might include the sources and a script
for ease-of-testing later).

# Error

This is to reproduce the error I'm facing.

First build neovim in the `neovim` directory. It contains the current master
plus https://github.com/neovim/neovim/pull/5269.

To reproduce the problem, run

```sh
LOG_LEVEL="debug" LOG_FILE=logs/log neovim/build/bin/nvim -u init.vim examples/Truck_Front_mid.pc
```

I nvim, run `:NvimPamConnect`, wait for >5s, then quit.

What you will get:
- The plugin log will be in `logs/log`, you can see some timeout error and a
  buffer reading error in there (I've saved that as `logs/log_mid`)
- In the `output` directory, the files `nvimpam.stdin` and `nvimpam.stdout` will
  contain the RPC communication of the plugin in binary form.

I've saved those output files as `nvimpam_mid.stdin` and `nvimpam_mid.stdout`.
I've decoded that stdout for this case in `nvimpam_mid.stdout.decoded`, which
looks fine. Stdin seems not to be decodable without errors.


For comparison, the plugin works when running

```sh
LOG_LEVEL="debug" LOG_FILE=logs/log neovim/build/bin/nvim -u init.vim examples/Truck_Front.pc
```

Also run `:NvimPamConnect`. Neovim will quit (the plugin does that), and produce
the same files as above (I've left those under their original name). There's no
timeout in `logs/log`, and you can see in `output/nvimpam.stdout.decoded` that
the plugin really does what it should. Stdin isn't decodable as well, though.

## License

Dual-Licensed under Apache or MIT at your leisure, see the LICENSE-\* files.
