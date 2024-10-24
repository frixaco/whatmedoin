## TODO

- [ ] Turn `cli` into CLI app and a daemon
  - [ ] `cli start` should start a background service on macOS, Windows, Linux
  - [ ] `cli stop` should stop the background service on macOS, Windows, Linux
-

- [ ] Make it more "public":

  - [ ] Allow setting application names to track
  - [ ] Allow setting browser tab titles and urls to track

## Setup

- Install bun, rustup, cargo-watch, Railway CLI
- `bun install`
- Set env variables in `.env` for each app

## CLI

- `cd apps/cli`
- `bun run build`
- Set `API_URL` in `.env` and update `run-cli.sh` if needed
- `crontab -e` and type `* * * * * /Users/frixaco/personal/whatmedoin/apps/cli/run-cli.sh`

## Chrome & Firefox extensions:

**NOTE**: Correct Chrome executable path, `openssl` and `ouch` CLI tools are required (or use alternatives)

- `cd apps/browser`
- `openssl genrsa -out key.pem 2048`
- `bun build:chrome` and `/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --pack-extension=./chrome --pack-extension-key=./key.pem` to generate `chrome.crx`
- `bun build:firefox` and `ouch c firefox/* firefox.zip && mv firefox.zip firefox.xpi` to generate `firefox.xpi`

## Research - enable cross compilation using `cross-rs` between macOS, Windows, Linux

<details>
  <summary>Toolchain setup</summary>

- `rustup default stable`
- `cargo install cross`
- ~~`rustup target add aarch64-apple-darwin`~~ macOS needed
- `rustup toolchain install stable-x86_64-pc-windows-gnu --force-non-host`
- `rustup toolchain install stable-x86_64-unknown-linux-gnu --force-non-host`

</details>

<details>
  <summary>Helper scripts</summary>

- `"build:windows": "cross build --target x86_64-pc-windows-gnu --release && cp target/x86_64-pc-windows-gnu/release/cli.exe ./cli-windows.exe"`
- `"build:linux": "cross build --target x86_64-unknown-linux-gnu --release && cp target/x86_64-unknown-linux-gnu/release/cli ./cli-linux"`
- `"build:linuxarm": "cross build --target aarch64-unknown-linux-gnu --release && cp target/aarch64-unknown-linux-gnu/release/cli ./cli-linuxarm"`
- `"build:macos": "cross build --target aarch64-apple-darwin --release && cp target/aarch64-apple-darwin/release/cli ./cli-macos"`

</details>

## Data shapes

- `type` - `browser_tab`, `app`, `phone`

- Learning Japanese
  - `url` (guidetojapanese.org)
- Watching YouTube & Listening to Music playlist
  - `url` (`&list=")
  - `title`
- Blender, Arc, Cursor, WezTerm, Steam, Hoyoplay, WuWa, ... open
  - `title`
- Using my phone
  - `type` - `phone`

## Tracks

- macOS, Windows, Linux

  - If I'm using certain apps - current foreground window
    - Blender
    - Arc
      These require browser extension:
      - Learning Japanese
      - Listening to Music playlist on YouTube
      - Watching YouTube
    - Cursor
    - WezTerm
    - Games are open (Steam, Hoyoplay, WuWa, ...)

- Android/iOS - If requests are coming to server, then I'm on my phone

- No update means I'm AFK

## Solutions tried

- macOS, Windows, Linux:
  - Tauri with x-win crate - crashes for some reason
  - NPM lib: https://github.com/paymoapp/active-window
  - Bun + C => executable
  - Flutter
  - Rust CLI with x-win [✓]
- Android/iOS:
  - React Native app that's always running
  - Flutter [✓]
- API
  - Bun [✓]
  - Rust + Tide
- Browser extension
  - Vanilla JS with Bun [✓]
  - Rust + WASM
