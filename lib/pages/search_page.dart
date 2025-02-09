import 'package:fixnum/fixnum.dart';
import 'package:flutter/material.dart';
import 'package:spout_social/messages/all.dart';

class SearchPage extends StatefulWidget {
  const SearchPage({super.key});

  @override
  _SearchPageState createState() => _SearchPageState();
}

class _SearchPageState extends State<SearchPage> {
  String _searchType = 'users';

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Search'),
      ),
      body: StreamBuilder(
          stream: ProfileListResponse.rustSignalStream,
          builder: (context, snapshot) {
            final rustSignal = snapshot.data;

            return Padding(
              padding: const EdgeInsets.all(16.0),
              child: Column(
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      ChoiceChip(
                        label: Text('Users'),
                        selected: _searchType == 'users',
                        onSelected: (selected) {
                          setState(() {
                            _searchType = 'users';
                          });
                        },
                      ),
                      SizedBox(width: 10),
                      ChoiceChip(
                        label: Text('Posts'),
                        selected: _searchType == 'posts',
                        onSelected: (selected) {
                          setState(() {
                            _searchType = 'posts';
                          });
                        },
                      ),
                    ],
                  ),
                  SizedBox(height: 20),
                  TextField(
                    decoration: InputDecoration(
                      labelText: 'Search',
                      border: OutlineInputBorder(),
                    ),
                    onSubmitted: (query) {
                      // Implement search logic here
                    },
                  ),
                  SizedBox(height: 20),
                  OutlinedButton(
                      onPressed: () {
                        if (_searchType == 'users') {
                          ProfileListRequestQuery(amount: Int64(100), offset: Int64(0)).sendSignalToRust();
                          print("Sent request to rust");
                        } else if (_searchType == 'posts') {
                          // Implement search logic for posts here
                        }
                      },
                      child: Text("Search")),
                  Builder(
                    builder: (context) {
                      if (rustSignal == null) {
                        return SizedBox();
                      } else {
                        final profiles = rustSignal.message.profiles;
                        return Expanded(
                          child: ListView.builder(
                            itemCount: profiles.length,
                            itemBuilder: (context, index) {
                              final profile = profiles[index];
                              return ListTile(
                                title: Text(profile.name),
                                subtitle: Text(profile.handle),
                              );
                            },
                          ),
                        );
                      }
                    },
                  ),
                ],
              ),
            );
          }),
    );
  }
}
