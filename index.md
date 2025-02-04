# spout_social

A decentralized social networking system for building tight-knit communities.

## Model and Terminology

Cell
: A distinct community

Coordinator
: The node that coordinates all cell activity. The source-of-truth for this cell.

Moderator
: A node authorized to take moderation actions within a cell. Moderators are assigned by the cell coordinator.

Post
: A top-level submission to a cell's feed. Posts can have a `title` and a `body`

Comment
: A text-based response to a `post`.

Reaction
: An emoji-based response to a `comment` or `post`

