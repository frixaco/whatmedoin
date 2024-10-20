import 'package:plugin_platform_interface/plugin_platform_interface.dart';

import 'foreground_window_title_plugin_method_channel.dart';

abstract class ForegroundWindowTitlePluginPlatform extends PlatformInterface {
  /// Constructs a ForegroundWindowTitlePluginPlatform.
  ForegroundWindowTitlePluginPlatform() : super(token: _token);

  static final Object _token = Object();

  static ForegroundWindowTitlePluginPlatform _instance = MethodChannelForegroundWindowTitlePlugin();

  /// The default instance of [ForegroundWindowTitlePluginPlatform] to use.
  ///
  /// Defaults to [MethodChannelForegroundWindowTitlePlugin].
  static ForegroundWindowTitlePluginPlatform get instance => _instance;

  /// Platform-specific implementations should set this with their own
  /// platform-specific class that extends [ForegroundWindowTitlePluginPlatform] when
  /// they register themselves.
  static set instance(ForegroundWindowTitlePluginPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  Future<String?> getPlatformVersion() {
    throw UnimplementedError('platformVersion() has not been implemented.');
  }
}
