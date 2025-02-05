import 'dart:convert';
import 'dart:io';

import 'package:fixnum/fixnum.dart';
import 'package:flutter/material.dart';

import 'package:image_picker/image_picker.dart';
import 'package:intl/intl.dart';
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
        "/settings": (BuildContext context) {
          return const SettingsPage();
        },
      },
    );
  }
}

class CreatePostPage extends StatefulWidget {
  const CreatePostPage({super.key});

  @override
  _CreatePostPageState createState() => _CreatePostPageState();
}

class _CreatePostPageState extends State<CreatePostPage> {
  final TextEditingController _bodyTextController = TextEditingController();
  final TextEditingController _titleTextController = TextEditingController();

  @override
  void dispose() {
    _bodyTextController.dispose();
    super.dispose();
  }

  Future<void> _createPost() async {
    final String bodyContent = _bodyTextController.text;
    final String titleContent = _titleTextController.text;

    CreatePostRequest(body: bodyContent, title: titleContent)
        .sendSignalToRust();
    _bodyTextController.clear();
    Navigator.pop(context);
  }

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
              spacing: 5.0,
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
                  controller: _titleTextController,
                  maxLines: 1,
                  autocorrect: true,
                  keyboardType: TextInputType.multiline,
                  decoration: InputDecoration(
                    border: OutlineInputBorder(),
                    hintText: 'Post Title',
                  ),
                ),
                TextField(
                  controller: _bodyTextController,
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
            ),
          ),
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
              onPressed: _createPost,
              child: Icon(Icons.send),
            ),
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
            Text(author,
                style: TextStyle(fontWeight: FontWeight.bold),
                textAlign: TextAlign.left),
            Text(date,
                style: TextStyle(color: Colors.grey, fontSize: 13.0),
                textAlign: TextAlign.left),
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
  String bio = "Bio";
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
                      style: TextStyle(fontSize: 14, color: Colors.grey),
                    ),
                    SizedBox(height: 10),
                    Text(
                      message.bio,
                      style: TextStyle(fontSize: 14),
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
                          SizedBox(height: 100),
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
                onPressed: () async {
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
                  // Add an artifical delay cause its kinda jarring how quickly it pops back over
                  // weird right?
                  await Future.delayed(const Duration(milliseconds: 100));
                  Navigator.pop(context);
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
            child: ListView.builder(
              itemCount: posts.length,
              itemBuilder: (BuildContext context, int index) {
                final post = posts[index];
                final dateFormat = DateFormat('dd/MM/yyyy HH:mm');
                return Post(
                  author: post.author,
                  date: dateFormat.format(DateTime.fromMillisecondsSinceEpoch(
                          post.createdAt.toInt())
                      .toLocal()),
                  title: post.title,
                  text: post.body,
                );
              },
            ));
      },
    );
  }
}

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  _HomePageState createState() => _HomePageState();
}

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
            OutlinedButton(onPressed: () {
              LogPostsTicket().sendSignalToRust();
            }, child: Text("Print Profile Ticket"))
          ],
        ),
      ),
    );
  }
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
