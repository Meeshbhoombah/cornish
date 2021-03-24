# Server Specefication

## User
Postgres

```
User {
    id
    email
    avatar_url
    display_name
    studios
}
```
- [ ] Create a user with a Google Account
- [ ] Get all user data
- [ ] Update user data

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


