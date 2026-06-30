use snotify;
use std::collections::HashMap;
 
 #[test]
fn save_and_load_playlist_test() {
    let mut songs: HashMap<String, snotify::Song> = HashMap::new();
    let id = String::from("id");
    let song = snotify::Song{
        name: Some(String::from("test_name")),
        artist: None,
        duration_ms: Some(300),
        user_data: vec![ 
            snotify::UserData{key: "key1".to_string(), value: "value1".to_string()},
            snotify::UserData{key: "key2".to_string(), value: "value2".to_string()},
        ],
    };

    songs.insert(id,song);

    let playlist_name = "test_playlist";
    let path = snotify::make_playlist_path(playlist_name);

    snotify::save_playlist(&path, &songs);
    
    let songs_desered = snotify::load_playlist(&path).unwrap();
    
    assert_eq!(songs, songs_desered);

    std::fs::remove_file(path).expect("Could not clean up file inside save_and_load_playlist_test");
}
