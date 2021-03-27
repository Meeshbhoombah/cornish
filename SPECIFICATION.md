# Specification
Requirements for the Cornish x Architect system.

- [ ] Networked Thinking
- [ ] Text-Channel
- [ ] Rooms
- [ ] Attributes
- [ ] Roles
- [ ] Serve a browser client
- [ ] Support authentication via Gmail
    + [ ] Only allow @meeshbhoombah and Cornish Faculty/Staff/Students

## Server
### Constraints
~1000 users
- Largest call size size? 
    + Presuming ~20% of persons in Room streaming video:
        * Idealistic = 800
        * Optimsitic = 100
        * Realistic = 65
        * Worst-Case = 25
    + Can get more with audio only streaming
        * Encourage the useage of an avatar

### main
- [ ] Get HOST and Port
    + [x] v0: Hardcoded
    + [ ] v1: Through CLI via clap
- [ ] Create a new `SocketAddr` from HOST and PORT
    + [x] v0: from Hardcoded
    + [ ] v1: from CLI
- [ ] Create a new `make_service_fn`
    + [ ] Get the raw `TcpStream` of the current connection via `socket`
        * `socket.into_inner();`
    + [ ] 
- [ ] Create a new `Server`
    + [ ] Bind server to created `SocketAddr` and `make_service_fn`
- [ ] Start Server
- [ ] Error Handling and Logging

### cornish 
- [ ] Check if connection received is HTTP or TCP
    + Use Alice's example, except match on uri
    + Pass raw TcpStream to tungstenite
- [ ] If HTTP, check serve index as a response
- [ ] If TCP, establish webrtc connection
    + [ ] SDP
    + [ ] ICE
    + [ ] STUN/TURN
- [ ] Send a pong request to an incoming ping request

### user
Postgres

Amazon S3 for storing profile photos

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
Image refs, accessible from S3

Graph root = IPFS DAG

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

### studio
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

### conversation
- Store Room data in Postgres
- RabbitMQ Message Queue

### room
Store Room data in Postgres
- [ ] Mesh <= 3 peers -> SFU w/ Mediasoup
- [ ] mutated SDP

Build simple, scalable tester.

## Client
### Constraints
- [ ] Support Chrome, Firefox, Safari, Edge
- [ ] Mobile Responsive - Safari/Chrome

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

### Conversation
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

### Room
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


