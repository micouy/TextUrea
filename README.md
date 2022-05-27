# TextUrea

## Installation

1. `cargo install tauri`
2. `git pull https://github.com/micouy/TextUrea.git`
3. `cd TextUrea`
4. `cargo tauri build`

## Usage

On macOS: After lauching the app, right click on its icon in the Dock, go to *Options*, then *Assign To: All Desktops*. Press `Cmd+C` while in TextUrea to hide it. **Note that this will overwrite your Clipboard.**

1. Copy any text to clipboard.
2. Press `` Cmd+Shift+` `` or `` Ctrl+Shift+` `` to focus TextUrea window and paste clipboard's content. The previous content in TextUrea will be overwritten.
3. Edit the text.
4. Press `Cmd+C` or `Ctrl+C`. The whole text will be copied and the focus will switch to the previous window.
