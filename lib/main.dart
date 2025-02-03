import 'package:flutter/material.dart';
import 'package:rinf/rinf.dart';
import './messages/all.dart';

void main() async {
  await initializeRust(assignRustSignal);
  runApp(const MainApp());
}

class MainApp extends StatelessWidget {
  const MainApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        body: Center(
          child: Column(
            children: [
              ElevatedButton(
                  onPressed: () async {
                    MyPreciousData(
                      inputNumbers: [3, 4, 5],
                      inputString: 'Zero-cost abstraction',
                    ).sendSignalToRust(); // GENERATED
                  },
                  child: const Text("Send dat message")),
              StreamBuilder(
                  stream: MyAmazingNumber.rustSignalStream,
                  builder: (context, snapshot) {
                    final rustSignal = snapshot.data;
                    if (rustSignal == null) {
                      return Text("Nothing received yet champ");
                    }
                    final myAmazingNumber = rustSignal.message;
                    final currentNumber = myAmazingNumber.currentNumber;
                    return Text(currentNumber.toString());
                  })
            ],
          ),
        ),
      ),
    );
  }
}
