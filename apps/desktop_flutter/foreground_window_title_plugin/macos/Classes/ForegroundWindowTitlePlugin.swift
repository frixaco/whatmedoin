import Cocoa
import FlutterMacOS

public class ForegroundWindowTitlePlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
    let channel = FlutterMethodChannel(name: "foreground_window_title_plugin", binaryMessenger: registrar.messenger)
    let instance = ForegroundWindowTitlePlugin()
    registrar.addMethodCallDelegate(instance, channel: channel)
  }

  public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
    switch call.method {
    case "getForegroundWindowTitle":
      result(getForegroundWindowTitle())
    case "requestPermissions":
      requestPermissions(result: result)
    default:
      result(FlutterMethodNotImplemented)
    }
  }

  private func getForegroundWindowTitle() -> String {
    if let app = NSWorkspace.shared.frontmostApplication {
      return app.localizedName ?? "Unknown"
    }
    return "Unknown"
  }

  private func requestPermissions(result: @escaping FlutterResult) {
    let options: NSDictionary = [kAXTrustedCheckOptionPrompt.takeUnretainedValue() as String: true]
    let accessibilityEnabled = AXIsProcessTrustedWithOptions(options)
    result(accessibilityEnabled)
  }
}
