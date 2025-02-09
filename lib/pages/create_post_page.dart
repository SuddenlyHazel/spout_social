import 'package:fixnum/fixnum.dart';
import 'package:flutter/material.dart';
import 'package:spout_social/messages/all.dart';

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
    Navigator.pop(context, "posts.updated");
    // Delay the execution to ensure the page transition is complete
    Future.delayed(Duration(milliseconds: 100), () {
      PostsRequestQuery(startAt: Int64(0), amount: 100).sendSignalToRust();
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(),
      body: SingleChildScrollView(
        child: Column(
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
                  SizedBox(
                    height: 10.0,
                  ),
                  Row(
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
                  )
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
