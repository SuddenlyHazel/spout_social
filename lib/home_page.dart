import 'package:flutter/material.dart';
import 'package:spout_social/main.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  _HomePageState createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title:
            Text("Spout Social", style: TextStyle(fontWeight: FontWeight.bold)),
        actions: [
          IconButton(onPressed: () {}, icon: const Icon(Icons.favorite)),
          IconButton(
              onPressed: () {
                Navigator.pushNamed(context, "/search");
              },
              icon: const Icon(Icons.search))
        ],
      ),
      floatingActionButton: Padding(
        padding: EdgeInsets.all(10.0),
        child: FloatingActionButton(
          onPressed: () {
            Navigator.pushNamed(context, "/create/post");
          },
          tooltip: "Create a post!",
          child: const Icon(Icons.edit_note),
        ),
      ),
      bottomNavigationBar: BottomAppBar(
        shape: CircularNotchedRectangle(),
        notchMargin: 6.0,
        child: Row(
          mainAxisSize: MainAxisSize.max,
          mainAxisAlignment: MainAxisAlignment.spaceAround,
          children: <Widget>[
            IconButton(
              icon: Icon(Icons.home),
              isSelected: true,
              color: Colors.deepPurple,
              onPressed: () {},
            ),
            IconButton(
              icon: Icon(Icons.mail),
              onPressed: () {},
            ),
            SizedBox(width: 48), // The dummy child
            IconButton(
              icon: Icon(Icons.settings),
              onPressed: () {
                Navigator.pushNamed(context, "/settings");
              },
            ),
            IconButton(
              icon: Icon(Icons.face),
              onPressed: () {
                Navigator.pushNamed(context, "/profile");
              },
            ),
          ],
        ),
      ),
      floatingActionButtonLocation: FloatingActionButtonLocation.centerDocked,
      body: PostFeed(),
    );
  }
}
