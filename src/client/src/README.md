# `client/src`
## ECS
Du is built with the ECS model.

### Styleguide
- `System/` (a directory)
- `Entity.rs`
- `component.rs`

# Window
User can access the Cornish system.

## Settings
User can select a theme.

User can set a font.

## Navigation
### Studio
User can navigate between their:
- Studio
- Collective Studios (which they have joined)
- Friends Collective
- A Collective Discovery Page

User can manage their Studios (sort them into groups).

User can create a Studio.

User can access the Window Settings.
 
### Building 
User can view information about the active Studio.

User can view and manage Levels.

User can navigate between:
- The active Collection
- Conversations
- Rooms

User can set the active Collections.

User can create a Conversation.

User can create a Level.

User can Go Live in a Room.

#### Levels
Grouped entities.

// - The Last Received Block ID
//
// This works because Blocks are ordered by their relations, and each new Block
// a user creates is related to the current Day Block, with each new Block
// being added as a tail to the list of Blocks.
//
// This is more effective than system timestamps as they're subject to
// randomness.
