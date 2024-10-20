#include "include/foreground_window_title_plugin/foreground_window_title_plugin_c_api.h"

#include <flutter/plugin_registrar_windows.h>

#include "foreground_window_title_plugin.h"

void ForegroundWindowTitlePluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar) {
  foreground_window_title_plugin::ForegroundWindowTitlePlugin::RegisterWithRegistrar(
      flutter::PluginRegistrarManager::GetInstance()
          ->GetRegistrar<flutter::PluginRegistrarWindows>(registrar));
}
