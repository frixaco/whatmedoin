import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:foreground_window_title_plugin/foreground_window_title_plugin_method_channel.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  MethodChannelForegroundWindowTitlePlugin platform = MethodChannelForegroundWindowTitlePlugin();
  const MethodChannel channel = MethodChannel('foreground_window_title_plugin');

  setUp(() {
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger.setMockMethodCallHandler(
      channel,
      (MethodCall methodCall) async {
        return '42';
      },
    );
  });

  tearDown(() {
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger.setMockMethodCallHandler(channel, null);
  });

  test('getPlatformVersion', () async {
    expect(await platform.getPlatformVersion(), '42');
  });
}
