import 'package:flutter/material.dart';
import 'screens/home_screen.dart';

void main() {
  runApp(const BridgeXApp());
}

class BridgeXApp extends StatelessWidget {
  const BridgeXApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'BridgeX',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        primarySwatch: Colors.blue,
        useMaterial3: true,
        brightness: Brightness.light,
      ),
      darkTheme: ThemeData(
        primarySwatch: Colors.blue,
        useMaterial3: true,
        brightness: Brightness.dark,
      ),
      themeMode: ThemeMode.system,
      home: const HomeScreen(),
    );
  }
}
