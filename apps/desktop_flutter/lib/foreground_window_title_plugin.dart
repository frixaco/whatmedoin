import 'dart:async';
import 'package:flutter/services.dart';

class ForegroundWindowTitlePlugin {
  static const MethodChannel _channel =
      MethodChannel('foreground_window_title_plugin');

  static Future<String> getForegroundWindowTitle() async {
    final String title =
        await _channel.invokeMethod('getForegroundWindowTitle');
    return title;
  }
}
