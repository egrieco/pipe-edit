# Basic Design

A TUI program that:

1. Takes input from `STDIN`.
2. Allows you to edit that input in a TUI.
3. Writes the output to `STDOUT`.

## Requirements

- Should be written in the latest version of Rust.
- Should include an appropriate `shell.nix` file with the dependencies necessary to build the project.
- TUI should be implemented via the `ratatui` and `tui-textarea` crates.
- `Shift+Enter` should trigger a succesful exit and write the modified buffer to `STDOUT`.
- `Esc` should present a dialog to the user with the options:
  - "Cancel" which dismisses the dialog.
  - "Abort" which exits with an error and no output.
  - "Pipe Out" which emits the edited buffer to `STDOUT`.
