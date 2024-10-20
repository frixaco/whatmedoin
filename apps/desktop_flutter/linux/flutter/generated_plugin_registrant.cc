//
//  Generated file. Do not edit.
//

// clang-format off

#include "generated_plugin_registrant.h"

#include <foreground_window_title_plugin/foreground_window_title_plugin.h>

void fl_register_plugins(FlPluginRegistry* registry) {
  g_autoptr(FlPluginRegistrar) foreground_window_title_plugin_registrar =
      fl_plugin_registry_get_registrar_for_plugin(registry, "ForegroundWindowTitlePlugin");
  foreground_window_title_plugin_register_with_registrar(foreground_window_title_plugin_registrar);
}
