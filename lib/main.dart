import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:image_picker/image_picker.dart';
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
      },
    );
  }
}

class CreatePostPage extends StatelessWidget {
  const CreatePostPage({super.key});
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(),
      body: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        mainAxisSize: MainAxisSize.max,
        children: [
          Padding(
              padding: const EdgeInsets.all(16.0),
              child: Column(
                spacing: 10.0,
                children: [
                  Row(
                    children: [
                      Text(
                        "Whatcha thinking about?",
                        style: TextStyle(fontSize: 23.0),
                        textAlign: TextAlign.left,
                      ),
                    ],
                  ),
                  TextField(
                    maxLines: null,
                    minLines: 4,
                    autocorrect: true,
                    keyboardType: TextInputType.multiline,
                    decoration: InputDecoration(
                      border: OutlineInputBorder(),
                      hintText: 'Write something wonderful!',
                    ),
                  ),
                ],
              ))
        ],
      ),
      bottomNavigationBar: BottomAppBar(
        shape: CircularNotchedRectangle(),
        child: Row(
          mainAxisSize: MainAxisSize.max,
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            OutlinedButton(
                style: OutlinedButton.styleFrom(
                  minimumSize: const Size(200, 50),
                ),
                onPressed: () {},
                child: Icon(Icons.send))
          ],
        ),
      ),
    );
  }
}

class Post extends StatelessWidget {
  const Post(
      {super.key,
      required this.author,
      required this.date,
      required this.title,
      required this.text});

  final String author;
  final String date;
  final String title;
  final String text;

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.all(10.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.start,
              children: [
                Text(author, style: TextStyle(fontWeight: FontWeight.bold)),
                Text(date, style: TextStyle(color: Colors.grey)),
              ],
            ),
          ),
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(title,
                    style:
                        TextStyle(fontSize: 18.0, fontWeight: FontWeight.bold)),
                SizedBox(height: 10.0),
                Text(text),
              ],
            ),
          ),
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.start,
              children: [
                Icon(Icons.favorite, color: Colors.red),
                SizedBox(width: 5.0),
              ],
            ),
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
    );
  }
}

class ProfilePage extends StatefulWidget {
  const ProfilePage({super.key});

  @override
  _ProfilePageState createState() => _ProfilePageState();
}

class _ProfilePageState extends State<ProfilePage> {
  String profileImageUrl =
      "https://images.unsplash.com/photo-1438761681033-6461ffad8d80?q=80&w=3540&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D";
  String username = "Username";
  String handle = "@handle";
  int postCount = 42;

  @override
  void initState() {
    super.initState();
    RequestUserProfile().sendSignalToRust();
  }

  ImageProvider _getImageProvider(profileImageUrl) {
    if (profileImageUrl.startsWith('http')) {
      return NetworkImage(profileImageUrl);
    } else if (profileImageUrl.startsWith('/')) {
      return FileImage(File(profileImageUrl));
    } else {
      final bytes = base64Decode(profileImageUrl);
      return MemoryImage(bytes);
    }
  }

  @override
  Widget build(BuildContext context) {
    return StreamBuilder(
        stream: ProfileSignal.rustSignalStream,
        builder: (context, snapshot) {
          final rustSignal = snapshot.data;
          if (rustSignal == null) {
            return Text("Loading..");
          }
          ProfileSignal message = rustSignal.message;

          return Scaffold(
            appBar: AppBar(),
            body: Center(
              child: Padding(
                padding: EdgeInsets.all(10.0),
                child: Column(
                  children: [
                    CircleAvatar(
                        radius: 50,
                        backgroundImage:
                            _getImageProvider(message.profileImage)),
                    SizedBox(height: 20),
                    Text(
                      message.name,
                      style:
                          TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
                    ),
                    Text(
                      message.handle,
                      style: TextStyle(fontSize: 18, color: Colors.grey),
                    ),
                    SizedBox(height: 20),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Column(
                          children: [
                            Text(
                              "Posts",
                              style: TextStyle(
                                  fontSize: 18, fontWeight: FontWeight.bold),
                            ),
                            Text(
                              postCount.toString(),
                              style: TextStyle(fontSize: 18),
                            ),
                          ],
                        ),
                      ],
                    ),
                    Padding(
                      padding: EdgeInsets.all(10.0),
                      child: Row(
                        mainAxisAlignment: MainAxisAlignment.center,
                        children: [
                          OutlinedButton(
                            onPressed: () {
// Update the navigation in ProfilePage to pass the values
                              Navigator.push(
                                context,
                                MaterialPageRoute(
                                  builder: (context) => EditProfilePage(
                                      profileImageUrl: message.profileImage,
                                      username: message.name,
                                      handle: message.handle,
                                      bio: message.bio,
                                      location: message
                                          .location // Add location field in ProfilePage if needed
                                      ),
                                ),
                              );
                            },
                            child: Text("Edit Profile"),
                          )
                        ],
                      ),
                    )
                  ],
                ),
              ),
            ),
          );
        });
  }
}

class EditProfilePage extends StatefulWidget {
  final String profileImageUrl;
  final String username;
  final String handle;
  final String bio;
  final String location;

  const EditProfilePage({
    super.key,
    required this.profileImageUrl,
    required this.username,
    required this.handle,
    required this.bio,
    required this.location,
  });

  @override
  _EditProfilePageState createState() => _EditProfilePageState();
}

class _EditProfilePageState extends State<EditProfilePage> {
  late String _profileImageUrl;
  final ImagePicker _picker = ImagePicker();

  late TextEditingController _usernameController;
  late TextEditingController _handleController;
  late TextEditingController _bioController;
  late TextEditingController _locationController;

  @override
  void initState() {
    super.initState();
    _profileImageUrl = widget.profileImageUrl;
    _usernameController = TextEditingController(text: widget.username);
    _handleController = TextEditingController(text: widget.handle);
    _bioController = TextEditingController(text: widget.bio);
    _locationController = TextEditingController(text: widget.location);
  }

  Future<void> _pickImage() async {
    final XFile? image = await _picker.pickImage(source: ImageSource.gallery);
    if (image != null) {
      final bytes = await File(image.path).readAsBytes();
      final base64String = base64Encode(bytes);
      setState(() {
        _profileImageUrl = base64String;
      });
    }
  }

  ImageProvider _getImageProvider() {
    if (_profileImageUrl.startsWith('http')) {
      return NetworkImage(_profileImageUrl);
    } else if (_profileImageUrl.startsWith('/')) {
      return FileImage(File(_profileImageUrl));
    } else {
      final bytes = base64Decode(_profileImageUrl);
      return MemoryImage(bytes);
    }
  }

  @override
  void dispose() {
    _usernameController.dispose();
    _handleController.dispose();
    _bioController.dispose();
    _locationController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(),
      body: Center(
        child: Padding(
          padding: EdgeInsets.all(10.0),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              GestureDetector(
                onTap: _pickImage,
                child: CircleAvatar(
                  radius: 50,
                  backgroundImage: _getImageProvider(),
                ),
              ),
              SizedBox(height: 20),
              TextField(
                decoration: InputDecoration(
                  labelText: "Name",
                  border: OutlineInputBorder(),
                ),
                controller: _usernameController,
              ),
              SizedBox(height: 20),
              TextField(
                decoration: InputDecoration(
                  labelText: "Handle",
                  border: OutlineInputBorder(),
                ),
                controller: _handleController,
              ),
              SizedBox(height: 20),
              TextField(
                decoration: InputDecoration(
                  labelText: "Bio",
                  border: OutlineInputBorder(),
                ),
                maxLines: 3,
                controller: _bioController,
              ),
              SizedBox(height: 20),
              TextField(
                decoration: InputDecoration(
                  labelText: "Location",
                  border: OutlineInputBorder(),
                ),
                controller: _locationController,
              ),
              SizedBox(height: 20),
              ElevatedButton(
                onPressed: () {
                  // You can now use these values to update the profile or send them to the server
                  String updatedUsername = _usernameController.text;
                  String updatedHandle = _handleController.text;
                  String updatedBio = _bioController.text;
                  String updatedLocation = _locationController.text;
                  UpdateUserProfile(
                          name: updatedUsername,
                          handle: updatedHandle,
                          bio: updatedBio,
                          location: updatedLocation,
                          profileImage: _profileImageUrl)
                      .sendSignalToRust();
                  // Handle save action
                },
                child: Text("Save"),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("queer.ooo"),
        actions: [
          IconButton(onPressed: () {}, icon: const Icon(Icons.favorite)),
          IconButton(onPressed: () {}, icon: const Icon(Icons.search))
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
              onPressed: () {},
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
      body: Center(
        child: Column(
          children: [
            const Post(
              author: "Hazel",
              date: "2023-10-10",
              title: "Sample Post",
              text: "This is a sample post to demonstrate the Post widget.",
            ),
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
                }),
            Text("Does hot reload work?!")
          ],
        ),
      ),
    );
  }
}
