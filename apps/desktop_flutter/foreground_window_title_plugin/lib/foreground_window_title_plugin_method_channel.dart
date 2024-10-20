import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

import 'foreground_window_title_plugin_platform_interface.dart';

/// An implementation of [ForegroundWindowTitlePluginPlatform] that uses method channels.
class MethodChannelForegroundWindowTitlePlugin extends ForegroundWindowTitlePluginPlatform {
  /// The method channel used to interact with the native platform.
  @visibleForTesting
  final methodChannel = const MethodChannel('foreground_window_title_plugin');

  @override
  Future<String?> getPlatformVersion() async {
    final version = await methodChannel.invokeMethod<String>('getPlatformVersion');
    return version;
  }
}
