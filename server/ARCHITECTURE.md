# Server Architecture

## User
Postgres

```
User {
    email
    display_name
    graph_root
    studios
}
```

### CREATE
- [ ] Create a user with a Google Account

### READ 
- [ ] Get all user data

## graph
LMDB

### CREATE
- [ ] Add a new block to a frame (default, #log)

### READ

## Studio
Studio {
    name
    image
    hero
    roles
    conversations
    rooms
    frames
}

### CREATE 
- [ ] Create a new Studio

### READ
- [ ] Get all Studio data

## Conversation
## Room

