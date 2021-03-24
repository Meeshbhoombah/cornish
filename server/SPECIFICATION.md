# Server Specefication

## User
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
    + [ ] 
- [ ] Get all user data
    + [ ] Get all data at inital loaad
- [ ] Update user data
    + [ ] Settings System can update `display_name`, `theme`, etc.

## graph
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
Store Room data in Postgres
Message Queue

## Room
Store Room data in Postgres

- [ ] Mesh <= 3 peers -> SFU w/ Mediasoup
- [ ] mutated SDP


