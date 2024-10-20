import 'package:flutter_test/flutter_test.dart';
import 'package:foreground_window_title_plugin/foreground_window_title_plugin.dart';
import 'package:foreground_window_title_plugin/foreground_window_title_plugin_platform_interface.dart';
import 'package:foreground_window_title_plugin/foreground_window_title_plugin_method_channel.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';

class MockForegroundWindowTitlePluginPlatform
    with MockPlatformInterfaceMixin
    implements ForegroundWindowTitlePluginPlatform {

  @override
  Future<String?> getPlatformVersion() => Future.value('42');
}

void main() {
  final ForegroundWindowTitlePluginPlatform initialPlatform = ForegroundWindowTitlePluginPlatform.instance;

  test('$MethodChannelForegroundWindowTitlePlugin is the default instance', () {
    expect(initialPlatform, isInstanceOf<MethodChannelForegroundWindowTitlePlugin>());
  });

  test('getPlatformVersion', () async {
    ForegroundWindowTitlePlugin foregroundWindowTitlePlugin = ForegroundWindowTitlePlugin();
    MockForegroundWindowTitlePluginPlatform fakePlatform = MockForegroundWindowTitlePluginPlatform();
    ForegroundWindowTitlePluginPlatform.instance = fakePlatform;

    expect(await foregroundWindowTitlePlugin.getPlatformVersion(), '42');
  });
}
