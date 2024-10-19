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
  - Tauri with x-win crate [x]
  - NPM lib: https://github.com/paymoapp/active-window
  - Bun + C => executable
- Android/iOS:
  - React Native app that's always running [x]

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
