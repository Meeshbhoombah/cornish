# Specification
Requirements for the Cornish x Architect system.

- [ ] Networked Thinking
    + [ ] Graph per Student
    + [ ] Shareable Blocks
- [ ] Text-Channel
    + [ ] Aggregator (w/ link to channel)
    + [ ] Thread side-view
- [ ] Rooms
    + [ ] Audio/Video
    + [ ] Avatar (coin or profile pic)
    + [ ] Built-in changeable text channel view w/ Aggregator default
    + [ ] Can Stream 
- [ ] Roles
- [ ] Attributes
- [ ] Individual/Departments
- [ ] Serve a browser client
    + [ ] Support Chrome, Firefox, Safari, Edge
    + [ ] Responsive - Safari/Chrome
- [ ] Support authentication via Gmail
    + [ ] Only allow @meeshbhoombah and Cornish Faculty/Staff/Students

## Server
### Constraints
**Load:** ~1000 users
- Largest call size size? 
    + Presuming ~20% of persons on video:
        * Idealistic = 800
        * Optimsitic = 100
        * Realistic = 65
        * Worst-Case = 25
    + Can get more with audio only streaming
        * Encourage the useage of an avatar

- [ ] Ability to serve a webrtc connection
    + Leverage webrtc-unreliable
    + Build simple, scalable tester
- [ ] Define User
    + [ ] Transport
    + [ ] Model
    + [ ] Consider

### User
Postgres

```
User {
    id
    email
    avatar_url
    display_name
    theme
    studios
}
```
- [ ] Create a user with a Google Account
    + During Onboard 
- [ ] Check if a user exists
    + On Client init
- [ ] Get all user data
    + On inital load 
- [ ] Update user data
    + Settings System can update `display_name`, `theme`, etc.

### graph
Store access data in Postgres
IPFS DAG

```
Block {
    id
    contents
}
```

```
Frame {
    id
    prev_frame_hash
    created_at
    name
}
```

- [ ] Add a new block to a frame (default, #log)

### Studio
Postgres

```
Studio {
    name
    avatar
    hero
    about

    roles
    members

    frames
    conversations
    rooms
}
```

- [ ] Create a new Studio
- [ ] Get all Studio data

### Conversation
Store Room data in Postgres
Message Queue

### Room
Store Room data in Postgres

- [ ] Mesh <= 3 peers -> SFU w/ Mediasoup
- [ ] mutated SDP

## Client
### User can navigate to cornish.app

### Landing 
- [ ] User can sign in with a cornish.edu email or Meeshbhoombah

### Onboard
- [ ] User can be onboarded through System customization
    + [ ] User Profile
        * [ ] Avatar
        * [ ] Display name
    + [ ] Theme
        * [ ] Light/Dark/System Preferences

### Window
#### Settings
- [ ] User can set Avatar
- [ ] User can change their Display name
- [ ] User can set the theme

#### Navigation (Building) 
- [ ] User can navigate between their Studio/Collective Studios
- [ ] User can access Window Settings

### Studio
#### Settings
##### Roles
- [ ] User can create a Role
- [ ] User can edit the Role's permissions

#### Elevator (Navigation)
- [ ] User can view information about the current Studio
- [ ] User can switch the current Timeline Frame
- [ ] User can expand the Building Navigation System to see more information
- [ ] User can view get a preview of the Active Members in Rooms
- [ ] User can navigate between the Timelime/Conversations/Rooms
- [ ] User can acceess the Studio settings

#### Conversation
- [ ] User can view an ongoing conversation (in real time)
- [ ] User can be notified of @mentions, @everybody, and @here
- [ ] User can view who is currency active in a conversation
- [ ] User can see notifications first
    + [ ] Appears right above the message bar, can be clicked to view context
- [ ] User can search

#### Summary
- [ ] User can see pinned messages
- [ ] User can see a collection of Threads
- [ ] User can see a Timeline of the Conversation

#### Room
- [ ] User can view a Room
- [ ] User can join the Room's call
- [ ] User can view the Room's Conversation (between Elevetor/Function)

#### Function
##### Input
- [ ] User can input...
    + [ ] [Markdown](https://www.markdownguide.org/cheat-sheet/)
- [ ] User can make Input full screen

##### Activity Bar
- [ ] User can get a short description of the currently Hovered item
- [ ] User can be notified of Errors and Messages (rainbow flash)
- [ ] User can be get information about the word count, etc

##### Output
- [ ] User can search 
    + [ ] Filters Frames, Blocks of the Active Frame
- [ ] User can squish/scroll through blocks created in the past
- [ ] User can get a preview of subblocks by clicking a block
    + [ ] Show the most recent Bullet and Todo
- [ ] User can view by Day, Week, Month, Quarter, or Year

#### View
- [ ] User can have tabs
- [ ] User can group tabs
- [ ] User can search through tabs and contents of pages (CMD+F)
- [ ] User can see currently selected Output block1
- [ ] User can make Viewer full screen

