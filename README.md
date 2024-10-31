# Simple app to track what I'm doing (almost) in real-time for macOS, Windows, Linux, Android, iOS, Chrome, Firefox

## Tracks

- macOS, Windows, Linux

  - If I'm using a browser:

    - Learning Japanese - TK guide, animelon, JP YT channels open
    - Listening to Music playlist on YouTube - "Music" playlist open
    - Watching YouTube - None of the above

  - If I'm using certain apps - current foreground window:

    - Blender
    - WezTerm
    - Cursor
    - Powershell
    - Games (osu!, Elden Ring, WuWa, ...)
    - Slack
    - Anki
    - Heptabase

- Android/iOS - If requests are coming to server, then I'm on my phone

- No update means I'm AFK

## Solutions tried

- macOS, Windows, Linux:
  - Tauri with x-win crate - crashes for some reason
  - NPM lib: https://github.com/paymoapp/active-window - works, wanted lightweight CLI and daemon solution
  - Bun + C => executable - works, wasn't sure how to turn it into a daemon
  - Flutter - works, couldn't get the foreground window detection working reliably, wanted lightweight CLI and daemon solution
  - Rust CLI with x-win [✓]
- Android/iOS:
  - Flutter - works and was easier to setup than RN
  - React Native app that's always running [✓]
- API
  - Bun + Hono [✓]
  - Rust + Tide - might switch later
- Browser extension
  - Vanilla JS/TS with Bun [✓]
  - Rust + WASM - have to compile to JS at the end, so no

## Setup

- Install Bun, rustup, cargo-watch, Railway CLI, Node.js
- `bun install`
- Set env variables in `.env` for each app
- For mobile app, follow React Native/Expo docs for Android Studio/Xcode setup

## CLI: [./apps/cli-unix](./apps/cli-unix) and [./apps/cli-windows](./apps/cli-windows)

- `bun run build`
- `./wmd start` for Linux/macOS
- `./wmd.exe install` to add to startup, `./wmd.exe start` to start, `./wmd.exe stop` to stop, `./wmd.exe uninstall` to remove from startup (requires admin)

## Chrome & Firefox extensions: [./apps/browser/chrome/](./apps/browser/chrome/) and [./apps/browser/firefox/](./apps/browser/firefox/)

**NOTE**: For Chrome, set correct Chrome executable path, make sure `openssl` and `ouch` CLI tools are available (or use alternatives)

- Run `bun dev:chrome` or `bun dev:firefox`, then "Load unpacked"/"Load temporary add-on" in Chrome/Firefox

- `openssl genrsa -out key.pem 2048` (Chrome requires this)
- `bun build:chrome` and `/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --pack-extension=./chrome --pack-extension-key=./key.pem` to generate `chrome.crx`
- `bun build:firefox` and `ouch c firefox/* firefox.zip && mv firefox.zip firefox.xpi` to generate `firefox.xpi`

## TODO

- [ ] Make it more "public":

  - [ ] Allow setting application names to track
  - [ ] Allow setting browser tab titles and urls to track

- [ ] Finish setting iOS app (Info.plist, ...)
- [ ] Check out Firefox add-on signing

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
