#ifndef FLUTTER_PLUGIN_FOREGROUND_WINDOW_TITLE_PLUGIN_H_
#define FLUTTER_PLUGIN_FOREGROUND_WINDOW_TITLE_PLUGIN_H_

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>

#include <memory>

namespace foreground_window_title_plugin {

class ForegroundWindowTitlePlugin : public flutter::Plugin {
 public:
  static void RegisterWithRegistrar(flutter::PluginRegistrarWindows *registrar);

  ForegroundWindowTitlePlugin();

  virtual ~ForegroundWindowTitlePlugin();

  // Disallow copy and assign.
  ForegroundWindowTitlePlugin(const ForegroundWindowTitlePlugin&) = delete;
  ForegroundWindowTitlePlugin& operator=(const ForegroundWindowTitlePlugin&) = delete;

  // Called when a method is called on this plugin's channel from Dart.
  void HandleMethodCall(
      const flutter::MethodCall<flutter::EncodableValue> &method_call,
      std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result);
};

}  // namespace foreground_window_title_plugin

#endif  // FLUTTER_PLUGIN_FOREGROUND_WINDOW_TITLE_PLUGIN_H_
