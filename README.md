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

## Keyboard Shortcuts

### Editor Mode
- `Alt+Enter`, `Ctrl+Enter`, `Shift+Enter`, `Ctrl+D` - Exit and pipe output to stdout
- `Ctrl+C`, `Ctrl+Q`, `Ctrl+W` - Exit without output (abort)
- `Esc` - Open exit options menu
- `Ctrl+F` - Open search bar
- `Ctrl+G` - Jump to next search match
- `Ctrl+Shift+G` - Jump to previous search match
- `Ctrl+J` - Join current line with the next line

### Search Mode
- Type to search (matches are highlighted in yellow)
- `Enter` - Jump to current match and return to editor
- `Ctrl+G` - Jump to next match
- `Ctrl+Shift+G` - Jump to previous match
- `Esc` - Cancel search and return to editor

## Caveats

The `--clipboard` flag to place the output on the clipboard does not work properly yet due to how ownership of the clipboard works in Linux. We'll probably have to spawn a background process to maintain the clipboard.

Take a look at [line 90 in wl-clipboard-rs/wl-clipboard-rs-tools/src/bin/wl-copy.rs](https://github.com/YaLTeR/wl-clipboard-rs/blob/master/wl-clipboard-rs-tools/src/bin/wl-copy.rs#L90) for some ideas.
