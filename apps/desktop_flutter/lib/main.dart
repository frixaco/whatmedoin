import 'dart:io';

import 'package:flutter/material.dart';
import 'dart:async';

import 'package:foreground_window_title_plugin/foreground_window_title_plugin.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  if (Platform.isMacOS) {
    try {
      await ForegroundWindowTitlePlugin.requestPermissions();
    } catch (e) {
      print('Failed to request permissions: $e');
    }
  }
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Window Title Display',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'Window Title Display'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  String _windowTitle = 'Unknown';
  late Timer _timer;

  @override
  void initState() {
    super.initState();
    _updateWindowTitle();
    // Set up a timer to update the window title every second
    _timer =
        Timer.periodic(const Duration(seconds: 1), (_) => _updateWindowTitle());
  }

  @override
  void dispose() {
    // Cancel the timer when the widget is disposed
    _timer.cancel();
    super.dispose();
  }

  Future<void> _updateWindowTitle() async {
    String windowTitle =
        await ForegroundWindowTitlePlugin.getForegroundWindowTitle();
    setState(() {
      _windowTitle = windowTitle;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            const Text(
              'Current foreground window title:',
            ),
            Text(
              _windowTitle,
              style: Theme.of(context).textTheme.headlineMedium,
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _updateWindowTitle,
        tooltip: 'Update',
        child: const Icon(Icons.refresh),
      ),
    );
  }
}
