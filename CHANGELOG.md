# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.3 (2026-04-04)

### New Features

 - <csr-id-91a111cbcb6d17471f2427246b07f2f69d97fa29/> add Alt+Up/Down keybindings to move lines up and down
 - <csr-id-de9fa1864e2182bbf5689fdfea246025212c67ca/> add scrollable help dialog with dynamic sizing
 - <csr-id-292257d5ec0c17ae4739dd8d99668294e1f5bf78/> add Ctrl+Alt+Backspace and Ctrl+Alt+Delete keybindings for buffer deletion
 - <csr-id-9151a1f90571925bce1a3e20058480ab93ba2647/> add help screen displayed with Ctrl+H showing keyboard shortcuts

### Bug Fixes

 - <csr-id-4ba5a1270f0708e1a0ed6cefcad3da75473836c4/> Correct minor typo
 - <csr-id-7ade69c0b2d01e6b411f5b2af0e6c42dff50f88a/> change delete buffer bindings from Ctrl+Shift to Ctrl+Alt
 - <csr-id-d7b5c2a9c2aec6e82de01139d2be2e7d77db9407/> update delete buffer keybindings and remove non-working Ctrl+Alt+Shift+Backspace
 - <csr-id-95817a6f98bbd29974846bfb6d83a4ef55673546/> change buffer deletion keybindings to avoid Niri conflict

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release.
 - 12 days passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Correct minor typo ([`4ba5a12`](https://github.com/egrieco/pipe-edit/commit/4ba5a1270f0708e1a0ed6cefcad3da75473836c4))
    - Add Alt+Up/Down keybindings to move lines up and down ([`91a111c`](https://github.com/egrieco/pipe-edit/commit/91a111cbcb6d17471f2427246b07f2f69d97fa29))
    - Add terminal keybinding info ([`24e2651`](https://github.com/egrieco/pipe-edit/commit/24e265145e96ad276c43493089bd5701a33fd58f))
    - Add scrollable help dialog with dynamic sizing ([`de9fa18`](https://github.com/egrieco/pipe-edit/commit/de9fa1864e2182bbf5689fdfea246025212c67ca))
    - Change delete buffer bindings from Ctrl+Shift to Ctrl+Alt ([`7ade69c`](https://github.com/egrieco/pipe-edit/commit/7ade69c0b2d01e6b411f5b2af0e6c42dff50f88a))
    - Update delete buffer keybindings and remove non-working Ctrl+Alt+Shift+Backspace ([`d7b5c2a`](https://github.com/egrieco/pipe-edit/commit/d7b5c2a9c2aec6e82de01139d2be2e7d77db9407))
    - Change buffer deletion keybindings to avoid Niri conflict ([`95817a6`](https://github.com/egrieco/pipe-edit/commit/95817a6f98bbd29974846bfb6d83a4ef55673546))
    - Add Ctrl+Alt+Backspace and Ctrl+Alt+Delete keybindings for buffer deletion ([`292257d`](https://github.com/egrieco/pipe-edit/commit/292257d5ec0c17ae4739dd8d99668294e1f5bf78))
    - Add help screen displayed with Ctrl+H showing keyboard shortcuts ([`9151a1f`](https://github.com/egrieco/pipe-edit/commit/9151a1f90571925bce1a3e20058480ab93ba2647))
</details>

## v0.1.2 (2026-03-23)

### New Features

 - <csr-id-ceb04b8d05965750ab0f1fdc873b97015a0aa1c1/> add `--question` flag with optional custom header support
 - <csr-id-a32e408e99201eb0e920df381cc75996093ffae8/> move editor instructions from title to bottom status bar
 - <csr-id-3dfe07c905316097d1dc84096182406a0f9034ad/> add Ctrl+f for next match and Ctrl+g for previous match in search mode
 - <csr-id-e957522cae5a7e7e185753eff6ebd875347a58aa/> add search functionality with Ctrl+F and match navigation
 - <csr-id-5ad42b641b61d54cd8fd37a2645d6314ee6b07e9/> add search functionality with Ctrl-F, Ctrl-G, and match highlighting
 - <csr-id-f83f5aba04272574164a1e4bbd390ea60738cd56/> add keybindings for exit with/without output
 - <csr-id-657d7141abe195f883a2acce20b998cde1cbcf6b/> add version flag with git commit information

### Bug Fixes

 - <csr-id-ed91bb1a1f70a1abd3af45b7f0dc76103d3f14d1/> enable search feature and fix KeyModifiers typo in tui-textarea
 - <csr-id-3d15f9b6c55d3ffc203142874f679fc540c9e1e4/> Undo some unnecessary changes and format

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 4 calendar days.
 - 4 days passed between releases.
 - 9 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release pipe-edit v0.1.2 ([`085a987`](https://github.com/egrieco/pipe-edit/commit/085a98730697222a1d1bcb45a031b9ddb490fced))
    - Add TOOD about keybindings ([`ff3131f`](https://github.com/egrieco/pipe-edit/commit/ff3131fc2e4c2720bafa2ac2312dda41208c271e))
    - Minor cleanup and formatting ([`f579d28`](https://github.com/egrieco/pipe-edit/commit/f579d286c5b8fd3ed267bbee0f72cad34d7bf52a))
    - Add `--question` flag with optional custom header support ([`ceb04b8`](https://github.com/egrieco/pipe-edit/commit/ceb04b8d05965750ab0f1fdc873b97015a0aa1c1))
    - Move editor instructions from title to bottom status bar ([`a32e408`](https://github.com/egrieco/pipe-edit/commit/a32e408e99201eb0e920df381cc75996093ffae8))
    - Add Ctrl+f for next match and Ctrl+g for previous match in search mode ([`3dfe07c`](https://github.com/egrieco/pipe-edit/commit/3dfe07c905316097d1dc84096182406a0f9034ad))
    - Update Cargo.lock ([`212d859`](https://github.com/egrieco/pipe-edit/commit/212d859a42e291dc195ee3b3f51d8916bda0406a))
    - Enable search feature and fix KeyModifiers typo in tui-textarea ([`ed91bb1`](https://github.com/egrieco/pipe-edit/commit/ed91bb1a1f70a1abd3af45b7f0dc76103d3f14d1))
    - Add search functionality with Ctrl+F and match navigation ([`e957522`](https://github.com/egrieco/pipe-edit/commit/e957522cae5a7e7e185753eff6ebd875347a58aa))
    - Add search functionality with Ctrl-F, Ctrl-G, and match highlighting ([`5ad42b6`](https://github.com/egrieco/pipe-edit/commit/5ad42b641b61d54cd8fd37a2645d6314ee6b07e9))
    - Add keybindings for exit with/without output ([`f83f5ab`](https://github.com/egrieco/pipe-edit/commit/f83f5aba04272574164a1e4bbd390ea60738cd56))
    - Undo some unnecessary changes and format ([`3d15f9b`](https://github.com/egrieco/pipe-edit/commit/3d15f9b6c55d3ffc203142874f679fc540c9e1e4))
    - Add version flag with git commit information ([`657d714`](https://github.com/egrieco/pipe-edit/commit/657d7141abe195f883a2acce20b998cde1cbcf6b))
</details>

## v0.1.1 (2026-03-19)

### New Features

 - <csr-id-484f728c1cf565aed74d1bf3912e13d81a80b916/> add --version flag with git commit and dirty status
 - <csr-id-0b486f42f8e6b6d01d1a05159367a1e2b2168af6/> add word deletion with Ctrl/Shift-Delete and Ctrl/Shift-Backspace
 - <csr-id-45c6777f689335b968de8b6518c7ff1e05956da4/> exit and output on Enter key in single-line mode
 - <csr-id-f50b052fa233149a74a650fbf4a9d89d60ebeb5a/> add `--single-line` / `-s` option to join lines and squeeze whitespace
 - <csr-id-3f468acb40e4ab8ae6f031238887c0288781ebd3/> add Ctrl-J keybinding to join current line with next line

### Bug Fixes

 - <csr-id-031de295bac6020f7385404fb7a76fb79b76b05b/> Add text that Aider missed
   Aider has a bug where applying patches that contain triple backticks terminate early.
 - <csr-id-23aa5f77e50b9454fe1708851012f6d0a23903aa/> handle alternative Ctrl-Backspace key codes from different terminals
 - <csr-id-61ab03dfaf77ffb63be27ad53a8de3c392ce80f9/> properly delete next line when joining lines

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release.
 - 26 days passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release pipe-edit v0.1.1 ([`bbc2aa1`](https://github.com/egrieco/pipe-edit/commit/bbc2aa1ccaabf61630212bc9be7f36f8b316ffec))
    - Add rust crates doc ([`8d8db1a`](https://github.com/egrieco/pipe-edit/commit/8d8db1a03ce9b3e12af9c06c7de80ee7dd903aea))
    - Add text that Aider missed ([`031de29`](https://github.com/egrieco/pipe-edit/commit/031de295bac6020f7385404fb7a76fb79b76b05b))
    - Add --version flag with git commit and dirty status ([`484f728`](https://github.com/egrieco/pipe-edit/commit/484f728c1cf565aed74d1bf3912e13d81a80b916))
    - Handle alternative Ctrl-Backspace key codes from different terminals ([`23aa5f7`](https://github.com/egrieco/pipe-edit/commit/23aa5f77e50b9454fe1708851012f6d0a23903aa))
    - Add word deletion with Ctrl/Shift-Delete and Ctrl/Shift-Backspace ([`0b486f4`](https://github.com/egrieco/pipe-edit/commit/0b486f42f8e6b6d01d1a05159367a1e2b2168af6))
    - Exit and output on Enter key in single-line mode ([`45c6777`](https://github.com/egrieco/pipe-edit/commit/45c6777f689335b968de8b6518c7ff1e05956da4))
    - Add `--single-line` / `-s` option to join lines and squeeze whitespace ([`f50b052`](https://github.com/egrieco/pipe-edit/commit/f50b052fa233149a74a650fbf4a9d89d60ebeb5a))
    - Properly delete next line when joining lines ([`61ab03d`](https://github.com/egrieco/pipe-edit/commit/61ab03dfaf77ffb63be27ad53a8de3c392ce80f9))
    - Add Ctrl-J keybinding to join current line with next line ([`3f468ac`](https://github.com/egrieco/pipe-edit/commit/3f468acb40e4ab8ae6f031238887c0288781ebd3))
</details>

## v0.1.0 (2026-02-20)

<csr-id-de425720050f737a46e3a0232e28848b118d335f/>

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

 - 17 commits contributed to the release.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release pipe-edit v0.1.0 ([`424394d`](https://github.com/egrieco/pipe-edit/commit/424394d96db89ca6093350c5992f238354fb48b7))
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

