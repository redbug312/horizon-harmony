import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:horizon_harmony/pages/home_page.dart';
import 'package:horizon_harmony/pages/track_info_page.dart';
import './messages/generated.dart';

void main() async {
  await initializeRust();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      scrollBehavior: MyCustomScrollBehavior(),
      theme: ThemeData(
        colorSchemeSeed: Colors.cyan,
        useMaterial3: true,
      ),
      home: const HomePage(),
      routes: {
        '/home': (context) => const HomePage(),
        '/track_info': (context) => const TrackInfoPage(),
      },
    );
  }
}

class MyCustomScrollBehavior extends MaterialScrollBehavior {
  // Override behavior methods and getters like dragDevices
  @override
  Set<PointerDeviceKind> get dragDevices => {
    PointerDeviceKind.touch,
    PointerDeviceKind.mouse,
  };
}
