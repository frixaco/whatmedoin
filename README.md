## Simple app to track what I'm doing (almost) in real-time for macOS, Windows, Linux, Chrome, Firefox (Android and iOS coming soon)

## Builds

- [./apps/cli-macos/whatmedoin.dmg](./apps/cli-macos/whatmedoin.dmg)
- [./apps/cli-windows/wmd.exe](./apps/cli-windows/wmd.exe)
- [./apps/browser/chrome](./apps/browser/chrome) and [./apps/browser/firefox](./apps/browser/firefox)

## TODO

- [ ] Make native Android and iOS apps
- [ ] Add rate-limiting (for browser, especially)
- [ ] Make it more "public":
  - [ ] Allow setting application names to track
  - [ ] Allow setting browser tab titles and urls to track

## Setup

- Install Bun, rustup, cargo-watch, Railway CLI, Node.js, Xcode
- `bun install`
- Set env variables in `.env` for each app
- For mobile app, follow React Native/Expo docs for Android Studio/Xcode setup

## Windows CLI: [./apps/cli-windows](./apps/cli-windows)

- `bun run build`
- add the `wmd.exe` to PATH
- (requires admin) `wmd install` to add to startup, `wmd start` to start, `wmd stop` to stop, `wmd uninstall` to remove from startup

## Chrome & Firefox extensions: [./apps/browser/chrome/](./apps/browser/chrome/) and [./apps/browser/firefox/](./apps/browser/firefox/)

**NOTE**: For Chrome, set correct Chrome executable path, make sure `openssl` and `ouch` CLI tools are available (or use alternatives)

<!-- - `openssl genrsa -out private_key.pem 2048` (Chrome requires this)
- `bun build:chrome` and `/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --pack-extension=./chrome --pack-extension-key=./private_key.pem` to generate `chrome.crx`
- to generate public key: `openssl rsa -in private_key.pem -pubout -out public_key.pem` -->

- For Chromium browser: Manage extensions => Load unpacked => select `chrome` folder
- For Firefox: Debug Add-ons => Load Temporary Add-on => select `firefox` folder
- Open the extension popup and set API URL

## macOS system tray app: [./apps/cli-macos/whatmedoin/](./apps/cli-macos/whatmedoin/)

- To create `whatmedoin.app`:
  - Open in the project in Xcode
  - Product > Archive > Distribute App > Custom > Clone App
- To package as `.dmg`, put `whatmedoin.app` inside `./cli-macos` and run `bun run package`

<!-- ## Android app: [./apps/mobile](./apps/mobile)

- https://reactnative.dev/docs/signed-apk-android
- Build an APK locally using either EAS, Android Studio or ./gradlew -->

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
  - Flutter - works, couldn't get the foreground window detection working reliably, also wanted lightweight CLI and daemon solution
  - Rust CLI with x-win (Windows) [✓]
  - Native macOS app (macOS) [✓]
- Android/iOS:
  - Flutter - works and was easier to setup than RN, might switch later
  - React Native app that's always running - couldn't get the background service working
  - Native Android app - too much work, might switch later
- API
  - Go + Echo [✓]
  - Bun + Hono - too much memory usage
  - Rust + Tide - might switch later
- Browser extension
  - Vanilla JS/TS with Bun [✓]
  - Rust + WASM - have to compile to JS at the end anyway

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
