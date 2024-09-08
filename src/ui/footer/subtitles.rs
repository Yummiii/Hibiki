use crate::{player::commons::Player, state::ArcPipe};
use gtk4::{prelude::EditableExt, Entry};

pub fn build_subtitles(state: ArcPipe<impl Player>) -> Entry {
    let subtitles = Entry::new();

    subtitles.connect_changed({
        let state = state.clone();
        move |entry| {
            let text = entry.text();
            let subtitle = text.parse::<i32>().unwrap_or(0);
            state.player.set_subtitle(subtitle);
        }
    });

    subtitles
}
