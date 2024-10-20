import 'dart:io';
import 'package:flutter/services.dart';

class ForegroundWindowTitlePlugin {
  static const MethodChannel _channel =
      MethodChannel('foreground_window_title_plugin');

  static Future<String> getForegroundWindowTitle() async {
    if (Platform.isMacOS || Platform.isWindows) {
      final String title =
          await _channel.invokeMethod('getForegroundWindowTitle');
      return title;
    }
    throw UnsupportedError('Unsupported platform');
  }

  static Future<void> requestPermissions() async {
    if (Platform.isMacOS) {
      await _channel.invokeMethod('requestPermissions');
    }
  }
}
