import Cocoa
import FlutterMacOS

public class WindowTitlePlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
    let channel = FlutterMethodChannel(name: "window_title_plugin", binaryMessenger: registrar.messenger)
    let instance = WindowTitlePlugin()
    registrar.addMethodCallDelegate(instance, channel: channel)
  }

  public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
    switch call.method {
    case "getForegroundWindowTitle":
      result(getForegroundWindowTitle())
    default:
      result(FlutterMethodNotImplemented)
    }
  }

  private func getForegroundWindowTitle() -> String {
    if let app = NSWorkspace.shared.frontmostApplication,
       let window = app.mainWindow {
      return window.title
    }
    return "Unknown"
  }
}
