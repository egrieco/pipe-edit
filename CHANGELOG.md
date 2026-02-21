# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.0 (2026-02-20)

### Chore

 - <csr-id-de425720050f737a46e3a0232e28848b118d335f/> update ratatui, tui-textarea, and crossterm dependencies

### New Features

 - <csr-id-b59c6b53d37fce664f94516ead5cd11f8e349f3f/> add --clipboard option using clap for CLI argument parsing
 - <csr-id-bb9cb73dd343922facb49a9700e013c1317a0086/> read initial content from clipboard when no piped input
 - <csr-id-cedf0353b5a9c7c7480ccc284bf345ab04b557bb/> add TUI text editor with stdin/stdout piping support

### Bug Fixes

 - <csr-id-b4faf32a6af24f472aa31e501333925b5dbfb59d/> downgrade ratatui and crossterm versions for tui-textarea compatibility

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update metadata and Rust edition ([`eca3efc`](https://github.com/egrieco/pipe-edit/commit/eca3efcc2089eb03b338cc25d30964986140995a))
    - Fix package name ([`30dcc96`](https://github.com/egrieco/pipe-edit/commit/30dcc96ea12cdfbc21d90c7706350e38012c935b))
    - Add caveats section and example code ([`e2de630`](https://github.com/egrieco/pipe-edit/commit/e2de6308024e19ef82bba766046d938575764389))
    - Correct program name and description ([`61ac809`](https://github.com/egrieco/pipe-edit/commit/61ac809d910a4df62dc2bef802bc8a70ddacd5fe))
    - Add --clipboard option using clap for CLI argument parsing ([`b59c6b5`](https://github.com/egrieco/pipe-edit/commit/b59c6b53d37fce664f94516ead5cd11f8e349f3f))
    - Enable wayland support ([`2c45186`](https://github.com/egrieco/pipe-edit/commit/2c451864eb968cf91402ea403750369cbfeb4f92))
    - Read initial content from clipboard when no piped input ([`bb9cb73`](https://github.com/egrieco/pipe-edit/commit/bb9cb73dd343922facb49a9700e013c1317a0086))
    - Update Cargo.lock ([`2112649`](https://github.com/egrieco/pipe-edit/commit/2112649b6e2836d86868d20f6674ba764c6bb587))
    - Downgrade ratatui and crossterm versions for tui-textarea compatibility ([`b4faf32`](https://github.com/egrieco/pipe-edit/commit/b4faf32a6af24f472aa31e501333925b5dbfb59d))
    - Update ratatui, tui-textarea, and crossterm dependencies ([`de42572`](https://github.com/egrieco/pipe-edit/commit/de425720050f737a46e3a0232e28848b118d335f))
    - Add Cargo.lock file ([`c4404a9`](https://github.com/egrieco/pipe-edit/commit/c4404a90ad7ad0d602ce9148f73e6bac4000b0e9))
    - Switch from Shift+Enter to Alt+Enter ([`c84faa2`](https://github.com/egrieco/pipe-edit/commit/c84faa2d73ac007b028a505a6af2eac32fc037c8))
    - Remove "To use" document ([`9cc6417`](https://github.com/egrieco/pipe-edit/commit/9cc6417e784968954d195ceb29a1080486f42afa))
    - Add TUI text editor with stdin/stdout piping support ([`cedf035`](https://github.com/egrieco/pipe-edit/commit/cedf0353b5a9c7c7480ccc284bf345ab04b557bb))
    - Add basic design docs ([`180f4a8`](https://github.com/egrieco/pipe-edit/commit/180f4a892cb4ef4972d088e6f1760e78e4352f56))
    - Initial Commit ([`30bbbe2`](https://github.com/egrieco/pipe-edit/commit/30bbbe2eda796df1eb1006f6ee6dcc70a7520f83))
</details>

