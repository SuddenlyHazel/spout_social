import 'dart:io';
import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:image_picker/image_picker.dart';
import 'package:spout_social/messages/all.dart';

class EditProfilePage extends StatefulWidget {
  final List<int> profileImageUrl;
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
  late List<int> _profileImageUrl;
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
      setState(() {
        _profileImageUrl = bytes;
      });
    }
  }

  ImageProvider _getImageProvider() {
    return MemoryImage(Uint8List.fromList(_profileImageUrl));
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
        child: SingleChildScrollView(
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
      ),
    );
  }
}
