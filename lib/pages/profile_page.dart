import 'package:flutter/material.dart';
import 'package:spout_social/messages/all.dart';
import 'package:spout_social/pages/edit_profile_page.dart';


class ProfilePage extends StatefulWidget {
  const ProfilePage({super.key});

  @override
  _ProfilePageState createState() => _ProfilePageState();
}

class _ProfilePageState extends State<ProfilePage> {
  List<int> profileImageUrl = List.empty();
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
    return MemoryImage(profileImageUrl);
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
