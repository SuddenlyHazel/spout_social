
import 'package:fixnum/fixnum.dart';
import 'package:flutter/material.dart';

import 'package:intl/intl.dart';
import 'package:rinf/rinf.dart';
import 'package:spout_social/home_page.dart';
import 'package:spout_social/pages/create_post_page.dart';
import 'package:spout_social/pages/profile_page.dart';
import 'package:spout_social/pages/settings_page.dart';
import 'package:spout_social/pages/search_page.dart';

import 'package:workmanager/workmanager.dart';
import './messages/all.dart';

@pragma(
    'vm:entry-point') // Mandatory if the App is obfuscated or using Flutter 3.1+
void callbackDispatcher() {
  Workmanager().executeTask((task, inputData) async {
    print(
        "Native called background task: $task"); //simpleTask will be emitted here.

    LogPostsTicket().sendSignalToRust();
    print("It looks like this also works");

    return Future.value(true);
  });
}

void main() async {
  await initializeRust(assignRustSignal);
  runApp(const MainApp());
}

class MainApp extends StatelessWidget {
  const MainApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          dynamicSchemeVariant: DynamicSchemeVariant.vibrant,
          seedColor: Colors.deepPurpleAccent,
          brightness: MediaQuery.platformBrightnessOf(context),
        ),
        useMaterial3: true,
      ),
      debugShowCheckedModeBanner: false,
      initialRoute: "/",
      routes: <String, WidgetBuilder>{
        "/": (BuildContext context) {
          return const HomePage();
        },
        "/create/post": (BuildContext context) {
          return const CreatePostPage();
        },
        "/profile": (BuildContext context) {
          return const ProfilePage();
        },
        "/settings": (BuildContext context) {
          return const SettingsPage();
        },
        "/search": (BuildContext content) {
          return SearchPage();
        },
      },
    );
  }
}

class Post extends StatelessWidget {
  const Post({
    super.key,
    required this.postId,
    required this.author,
    required this.date,
    required this.title,
    required this.text,
    required this.onDelete,
  });

  final String postId;
  final String author;
  final String date;
  final String title;
  final String text;
  final VoidCallback onDelete;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Container(
      decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(5),
          border: Border.all(
              color: theme.colorScheme.secondaryContainer, width: 2)),
      margin: const EdgeInsets.all(10.0),
      child: Padding(
        padding: EdgeInsets.symmetric(vertical: 5.0, horizontal: 10.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(author,
                        style: TextStyle(fontWeight: FontWeight.bold),
                        textAlign: TextAlign.left),
                    Text(date,
                        style: TextStyle(color: Colors.grey, fontSize: 13.0),
                        textAlign: TextAlign.left),
                  ],
                ),
                PopupMenuButton<String>(
                  onSelected: (String result) {
                    switch (result) {
                      case 'edit':
                        // Handle edit action
                        break;
                      case 'delete':
                        showDialog(
                            context: context,
                            builder: (BuildContext builder) {
                              final theme = Theme.of(context);

                              return AlertDialog(
                                  title: Text("Just checking.."),
                                  content: const Text(
                                      'Are you sure you want to delete this post? This cannot be undone, pal.'),
                                  actions: <Widget>[
                                    TextButton(
                                      onPressed: () =>
                                          Navigator.pop(context, 'Cancel'),
                                      child: const Text('Cancel'),
                                    ),
                                    FilledButton(
                                      style: FilledButton.styleFrom(
                                        backgroundColor:
                                            theme.colorScheme.error,
                                      ),
                                      onPressed: () {
                                        OwnerPostAction(
                                                postId: postId,
                                                action: OwnerPostAction_Action
                                                    .DELETE)
                                            .sendSignalToRust();
                                        onDelete();
                                        Navigator.pop(context, "Delete");
                                      },
                                      child: const Text('OK'),
                                    ),
                                  ]);
                            });
                        // Handle delete action
                        break;
                    }
                  },
                  itemBuilder: (BuildContext context) =>
                      <PopupMenuEntry<String>>[
                    const PopupMenuItem<String>(
                      value: 'edit',
                      child: Text('Edit'),
                    ),
                    const PopupMenuItem<String>(
                      value: 'delete',
                      child: Text('Delete'),
                    ),
                  ],
                ),
              ],
            ),
            SizedBox(height: 10.0),
            Text(title,
                style: TextStyle(fontSize: 18.0, fontWeight: FontWeight.bold)),
            SizedBox(height: 2.0),
            Text(text),
            SizedBox(height: 10.0),
            Row(
              mainAxisAlignment: MainAxisAlignment.start,
              children: [
                Icon(Icons.favorite, color: Colors.red),
                SizedBox(width: 5.0),
              ],
            ),
            OverflowBar(
              alignment: MainAxisAlignment.end,
              children: [
                IconButton(
                  icon: Icon(Icons.favorite_border),
                  onPressed: () {
                    // Handle like action
                  },
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

class PostFeed extends StatefulWidget {
  const PostFeed({super.key});

  @override
  _PostFeedState createState() => _PostFeedState();
}

class _PostFeedState extends State<PostFeed> {
  @override
  void initState() {
    super.initState();
    // Add your code here to execute when the widget is created
    _initializeHomePage();
  }

  Future<void> _initializeHomePage() async {
    // Example code to execute when the widget is created
    print("HomePage initialized");
    PostsRequestQuery(startAt: Int64(0), amount: 100).sendSignalToRust();

    // You can add more initialization code here
  }

  Future<void> _refreshPosts() async {
    // Implement your refresh logic here
    PostsRequestQuery(startAt: Int64(0), amount: 100).sendSignalToRust();
    print("Refreshed");
  }

  @override
  Widget build(BuildContext context) {
    return StreamBuilder(
      stream: PostQueryResponse.rustSignalStream,
      builder: (context, snapshot) {
        final rustSignal = snapshot.data;
        if (rustSignal == null) {
          return Text("Nothing received yet champ");
        }
        final posts = rustSignal.message.posts;
        return RefreshIndicator(
            onRefresh: _refreshPosts,
            child: posts.isEmpty
                ? Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Text("Nothing here yet.."),
                        OutlinedButton(
                            onPressed: _refreshPosts, child: Text("Refresh"))
                      ],
                    ),
                  )
                : ListView.builder(
                    itemCount: posts.length,
                    itemBuilder: (BuildContext context, int index) {
                      final post = posts[index];
                      final dateFormat = DateFormat('dd/MM/yyyy HH:mm');
                      return Post(
                        postId: post.postId,
                        author: post.author,
                        date: dateFormat.format(
                            DateTime.fromMillisecondsSinceEpoch(
                                    post.createdAt.toInt())
                                .toLocal()),
                        title: post.title,
                        text: post.body,
                        onDelete: _refreshPosts,
                      );
                    },
                  ));
      },
    );
  }
}