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
              title: Text("公館站 - 松山站"),
              subtitle: Text("營運時間已過"),
            ),
          ),
          Card(
            child: ListTile(
              title: Text("公館站 - 新店站"),
              subtitle: Text("營運時間已過"),
            ),
          ),
        ]
      )
    );
  }
}
