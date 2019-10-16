mod playlist;

use super::app::App;
use termion::event::Key;

pub fn handle_app(key: Key, app: &mut App) {
    match key {
        Key::Char('a') => {
            if let Some(current_playback_context) = &app.current_playback_context {
                if let Some(full_track) = &current_playback_context.item.clone() {
                    app.get_album_tracks(full_track.album.clone());
                }
            };
        }
        Key::Char('-') => {
            app.decrease_volume();
        }
        Key::Char('+') => {
            app.increase_volume();
        }
        _ => handle_block_events(key, app),
    }
}

// handle current block events
fn handle_block_events(key: Key, app: &mut App) {

    // get current route
    let current_route = app.get_current_route();

    match current_route.active_block {
        ActiveBlock::MyPlaylists => {
            playlist::handler(key, app);
        }
    }
}