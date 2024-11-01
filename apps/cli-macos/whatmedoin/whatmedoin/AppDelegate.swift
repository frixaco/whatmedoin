import Cocoa
import ApplicationServices

class AppDelegate: NSObject, NSApplicationDelegate {
    var statusItem: NSStatusItem?
    var timer: Timer?
    private var isMonitoring = true
    private var lastWindowInfo: (title: String?, appName: String?, path: String?)? // Store last window info
    private var lastWindowMenuItem: NSMenuItem? // Reference to menu item
    
    func applicationDidFinishLaunching(_ aNotification: Notification) {
        setupStatusBarItem()
        requestPermissions()
        startWindowTitleMonitoring()
    }
    
    func applicationWillTerminate(_ aNotification: Notification) {
        timer?.invalidate()
        timer = nil
        
        if let statusItem = statusItem {
            NSStatusBar.system.removeStatusItem(statusItem)
        }
    }
    
    func requestPermissions() {
        let options: NSDictionary = [kAXTrustedCheckOptionPrompt.takeUnretainedValue() as String: true]
        AXIsProcessTrustedWithOptions(options)
    }
    
    func getForegroundWindowInfo() -> (title: String?, appName: String?, path: String?) {
        guard let app = NSWorkspace.shared.frontmostApplication else {
            return (nil, nil, nil)
        }
        
        let appName = app.localizedName
        let appPath = app.bundleURL?.path
        
        let appRef = AXUIElementCreateApplication(app.processIdentifier)
        
        var windowRef: CFTypeRef?
        let result = AXUIElementCopyAttributeValue(appRef, kAXFocusedWindowAttribute as CFString, &windowRef)
        
        if result == .success, let windowRef = windowRef {
            var titleRef: CFTypeRef?
            let titleResult = AXUIElementCopyAttributeValue(windowRef as! AXUIElement, kAXTitleAttribute as CFString, &titleRef)
            
            if titleResult == .success, let titleRef = titleRef {
                let title = titleRef as? String
                return (title, appName, appPath)
            }
        }
        
        return (nil, appName, appPath)
    }
    
    func startWindowTitleMonitoring() {
        timer = Timer.scheduledTimer(withTimeInterval: 15 * 60, repeats: true) { [weak self] _ in
            guard let self = self, self.isMonitoring else { return }
            
            let windowInfo = self.getForegroundWindowInfo()
            self.lastWindowInfo = windowInfo
            
            let dateFormatter = DateFormatter()
            dateFormatter.dateFormat = "yyyy-MM-dd HH:mm:ss"
            let timestamp = dateFormatter.string(from: Date())
            
            if let appName = windowInfo.appName {
                let title = windowInfo.title ?? "No Title"
                let path = windowInfo.path ?? "No Path"
                print("[\(timestamp)] App: \(appName) - Window: \(title) - Path: \(path)")
                
                let trackedApps = ["WezTerm", "Cursor", "Slack", "Anki", "Heptabase"]
                if trackedApps.contains(appName) {
                    self.sendToAPI(title: appName, url: path)
                }
                
                DispatchQueue.main.async {
                    self.updateLastWindowMenuItem()
                }
            }
        }
    }
    
    func setupStatusBarItem() {
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.squareLength)
        
        if let button = statusItem?.button {
            if let image = NSImage(systemSymbolName: "eye", accessibilityDescription: "Window Tracker") {
                image.isTemplate = true
                button.image = image
            } else {
                button.title = "👁️"
            }
        }
        
        setupMenu()
    }
    
    
    func setupMenu() {
        let menu = NSMenu()
        
        lastWindowMenuItem = NSMenuItem(title: "Last window: None", action: nil, keyEquivalent: "")
        menu.addItem(lastWindowMenuItem!)
        
        menu.addItem(NSMenuItem.separator())
        
        let toggleItem = NSMenuItem(title: "Monitoring: On", action: #selector(toggleMonitoring), keyEquivalent: "t")
        menu.addItem(toggleItem)
        
        menu.addItem(NSMenuItem.separator())
        
        menu.addItem(NSMenuItem(title: "Quit", action: #selector(NSApplication.terminate(_:)), keyEquivalent: "q"))
        
        statusItem?.menu = menu
    }
    
    func updateLastWindowMenuItem() {
        if let menuItem = lastWindowMenuItem {
            if let windowInfo = lastWindowInfo,
               let appName = windowInfo.appName {
                let title = windowInfo.title ?? "No Title"
                let path = windowInfo.path ?? "No Path"
                menuItem.title = "Last window: \(appName) - \(title)\nPath: \(path)"
            } else {
                menuItem.title = "Last window: None"
            }
        }
    }
    
    @objc func toggleMonitoring() {
        isMonitoring.toggle()
        if let menuItem = statusItem?.menu?.items.first(where: { $0.action == #selector(toggleMonitoring) }) {
            menuItem.title = "Monitoring: \(isMonitoring ? "On" : "Off")"
        }
    }
    
    func sendToAPI(title: String, url: String) {
        guard let apiUrl = URL(string: "https://api.frixaco.com/activity") else { return }
        
        var request = URLRequest(url: apiUrl)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        
        let payload: [String: Any] = [
            "title": title,
            "url": url,
            "platform": "macos"
        ]
        
        do {
            request.httpBody = try JSONSerialization.data(withJSONObject: payload)
        } catch {
            print("Error creating JSON: \(error)")
            return
        }
        
        URLSession.shared.dataTask(with: request) { data, response, error in
            if let error = error {
                print("Error sending data: \(error)")
                return
            }
            
            if let httpResponse = response as? HTTPURLResponse {
                print("API Response: \(httpResponse.statusCode)")
            }
        }.resume()
    }
}
