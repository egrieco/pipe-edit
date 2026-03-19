# Pipe Editor

A TUI program that takes input from `STDIN`, allows you to edit that input, and then writes the output to `STDOUT`.

## Usage

```bash
# Edit piped input
echo "some text" | pipe-edit

# Edit clipboard contents
pipe-edit

# Output to clipboard instead of stdout
echo "some text" | pipe-edit --clipboard

# Single-line mode (joins lines, squeezes whitespace)
echo "some text" | pipe-edit --single-line

# Show version information (includes git commit and dirty status)
pipe-edit --version
```

## Caveats

The `--clipboard` flag to place the output on the clipboard does not work properly yet due to how ownership of the clipboard works in Linux. We'll probably have to spawn a background process to mainta
in the clipboard.

Take a look at [line 90 in wl-clipboard-rs/wl-clipboard-rs-tools/src/bin/wl-copy.rs](https://github.com/YaLTeR/wl-clipboard-rs/blob/master/wl-clipboard-rs-tools/src/bin/wl-copy.rs#L90) for some ideas.

