import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import '../messages/station.pb.dart';

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
      body: StreamBuilder(
        stream: Schedule.rustSignalStream,
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.waiting) {
            return const CircularProgressIndicator();
          } else if (snapshot.connectionState == ConnectionState.done) {
            return const Text('Done');
          } else if (snapshot.hasError) {
            return const Text('Error');
          } else {
            final schedule = snapshot.data!.message;
            return ListView.builder(
              itemBuilder: (context, index) {
                final track = schedule.tracks[index];
                return Card(
                  child: ListTile(
                    title: Text("${schedule.station} - ${track.destination}"),
                    subtitle: Text(track.arrivalTime),
                  ),
                );
              },
              itemCount: schedule.tracks.length,
            );
          }
        },
      ),
    );
  }
}
