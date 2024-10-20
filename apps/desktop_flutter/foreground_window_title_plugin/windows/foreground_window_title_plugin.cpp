#include "foreground_window_title_plugin.h"

// This must be included before many other Windows headers.
#include <windows.h>

// For getPlatformVersion; remove unless needed for your plugin implementation.
#include <VersionHelpers.h>

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>
#include <flutter/standard_method_codec.h>

#include <memory>
#include <sstream>

namespace foreground_window_title_plugin
{

  // static
  void ForegroundWindowTitlePlugin::RegisterWithRegistrar(
      flutter::PluginRegistrarWindows *registrar)
  {
    auto channel =
        std::make_unique<flutter::MethodChannel<flutter::EncodableValue>>(
            registrar->messenger(), "foreground_window_title_plugin",
            &flutter::StandardMethodCodec::GetInstance());

    auto plugin = std::make_unique<ForegroundWindowTitlePlugin>();

    channel->SetMethodCallHandler(
        [plugin_pointer = plugin.get()](const auto &call, auto result)
        {
          plugin_pointer->HandleMethodCall(call, std::move(result));
        });

    registrar->AddPlugin(std::move(plugin));
  }

  ForegroundWindowTitlePlugin::ForegroundWindowTitlePlugin() {}

  ForegroundWindowTitlePlugin::~ForegroundWindowTitlePlugin() {}

  std::string getForegroundWindowTitle()
  {
    HWND hwnd = GetForegroundWindow();
    if (hwnd)
    {
      int length = GetWindowTextLengthW(hwnd);
      if (length > 0)
      {
        std::vector<wchar_t> buffer(length + 1);
        GetWindowTextW(hwnd, &buffer[0], length + 1);

        int size_needed = WideCharToMultiByte(CP_UTF8, 0, &buffer[0], -1, NULL, 0, NULL, NULL);
        std::string strTo(size_needed, 0);
        WideCharToMultiByte(CP_UTF8, 0, &buffer[0], -1, &strTo[0], size_needed, NULL, NULL);
        return strTo;
      }
    }
    return "Unknown";
  }

  void ForegroundWindowTitlePlugin::HandleMethodCall(
      const flutter::MethodCall<flutter::EncodableValue> &method_call,
      std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result)
  {
    if (method_call.method_name().compare("getForegroundWindowTitle") == 0)
    {
      result->Success(flutter::EncodableValue(getForegroundWindowTitle()));
    }
    else
    {
      result->NotImplemented();
    }
  }

} // namespace foreground_window_title_plugin
