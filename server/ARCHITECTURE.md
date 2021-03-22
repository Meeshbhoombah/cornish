# Server Architecture

## User
Postgres

```
User {
    email
    display_name
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

### CREATE 
- [ ] Create a new Studio

### READ
- [ ] Get all Studio data

## Conversation
## Room

