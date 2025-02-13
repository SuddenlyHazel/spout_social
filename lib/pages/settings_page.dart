import 'package:flutter/material.dart';
import 'package:spout_social/messages/all.dart';

class SettingsPage extends StatefulWidget {
  const SettingsPage({super.key});

  @override
  _SettingsPageState createState() => _SettingsPageState();
}

class _SettingsPageState extends State<SettingsPage> {
  bool _notificationsEnabled = true;
  String _selectedTheme = 'Light';

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Settings'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            SwitchListTile(
              title: Text('Enable Notifications'),
              value: _notificationsEnabled,
              onChanged: (bool value) {
                setState(() {
                  _notificationsEnabled = value;
                });
              },
            ),
            SizedBox(height: 20),
            Text('Select Theme'),
            ListTile(
              title: const Text('Light'),
              leading: Radio<String>(
                value: 'Light',
                groupValue: _selectedTheme,
                onChanged: (String? value) {
                  setState(() {
                    _selectedTheme = value!;
                  });
                },
              ),
            ),
            ListTile(
              title: const Text('Dark'),
              leading: Radio<String>(
                value: 'Dark',
                groupValue: _selectedTheme,
                onChanged: (String? value) {
                  setState(() {
                    _selectedTheme = value!;
                  });
                },
              ),
            ),
            OutlinedButton(
                onPressed: () {
                  LogPostsTicket().sendSignalToRust();
                },
                child: Text("Print Profile Ticket")),
            OutlinedButton(
              onPressed: () async {
                ListNodes.create().sendSignalToRust();
              },
              child: Text("List Nodes"),
            ),
            StreamBuilder(
                stream: CurrentNodes.rustSignalStream,
                builder: (context, snapshot) {
                  final rustSignal = snapshot.data;
                  if (rustSignal == null) {
                    return Text("Nothing received yet champ");
                  }
                  final nodes = rustSignal.message.nodes;
                  if (nodes.isEmpty) {
                    return Text("No connected peers");
                  }
                  return Expanded(
                    child: ListView.builder(
                        itemCount: nodes.length,
                        itemBuilder: (context, idx) {
                          final node = nodes[idx];
                          return ListTile(title: Text(node));
                        }),
                  );
                })
          ],
        ),
      ),
    );
  }
}
