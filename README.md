# Pipe Editor

A TUI program that takes input from `STDIN`, allows you to edit that input, and then writes the output to `STDOUT`.

## Caveats

The `--clipboard` flag to place the output on the clipboard does not work properly yet due to how ownership of the clipboard works in Linux. We'll probably have to spawn a background process to maintain the clipboard.

Take a look at [line 90 in wl-clipboard-rs/wl-clipboard-rs-tools/src/bin/wl-copy.rs](https://github.com/YaLTeR/wl-clipboard-rs/blob/master/wl-clipboard-rs-tools/src/bin/wl-copy.rs#L90) for some ideas.
