# Snotify
This project aims to create a note display client for spotify playlists and bundles a number of apps to help with development.

- Snotify (main app): poll spotify for current song and display notes from the song(playlist) database for the currentty playing song id
```
    cargo run --bin snotify <playlist_name>
```
The playlist_name srgument specifies the database file to load.


- Record: update the specified database (playlist) with notes for the currently playing song id
```
    cargo run --bin record <playlist_name> <key1> <value1> <key2> <value2> ...
```
the playlist name must be followed by an even number of arguments, which will stored as arbitrary key-value pairs in the UserData section of the song info inside the song database.

- Mock replay client: 
- Mock replay server: 
- Mock replay local: 

