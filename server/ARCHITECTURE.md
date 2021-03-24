# Server Architecture

## User
Postgres

```
User {
    id
    email
    display_name
    studios
}
```
- [ ] Create a user with a Google Account
- [ ] Get all user data

## graph
IPFS dag

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

## Studio
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

## Conversation
Message queue

## Room
- [ ] MCU
- [ ] STCP connection
- [ ] ICE - establish realtime connection
- [ ] STUN/TURN - get NAT status (public port OR means to reach port from priv)
- [ ] SDP - exchange singaling information (to notify others of who is important
       to connect w/)


