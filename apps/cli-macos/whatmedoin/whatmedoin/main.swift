import Cocoa

let app = NSApplication.shared
let delegate = AppDelegate()
app.delegate = delegate

// This line ensures the app runs as a background application
NSApp.setActivationPolicy(.accessory)

app.run()
