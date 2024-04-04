import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

class TrackInfoPage extends StatefulWidget {
  const TrackInfoPage({super.key});

  @override
  State<TrackInfoPage> createState() => TrackInfoState();
}

class TrackInfoState extends State<TrackInfoPage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("列車到站資訊"),
        centerTitle: true,
        foregroundColor: Theme.of(context).colorScheme.onPrimary,
        systemOverlayStyle: const SystemUiOverlayStyle(statusBarBrightness: Brightness.dark),
        leading: GestureDetector(
          child: IconButton(
            icon: Icon(Icons.arrow_back, color: Theme.of(context).colorScheme.onPrimary),
            onPressed: () {
              Navigator.pop(context);
            }
          ),
        ),
        flexibleSpace: Container(
          decoration: BoxDecoration(
            gradient: LinearGradient(
              begin: Alignment.centerLeft,
              end: Alignment.centerRight,
              colors: [Colors.lightBlue.shade900, Colors.teal.shade600],
            ),
          ),
        ),
      ),
      body: ListView(
        children: const [
          Card(
            child: ListTile(
              title: Text("松山機場站 - 南港展覽館站"),
              subtitle: Text("00:34"),
            ),
          ),
          Card(
            child: ListTile(
              title: Text("中山國中站 - 南港展覽館站"),
              subtitle: Text("09:47"),
            ),
          ),
          Card(
            child: ListTile(
              title: Text("南京復興站 - 南港展覽館站"),
              subtitle: Text("07:31"),
            ),
          ),
        ]
      )
    );
  }
}
