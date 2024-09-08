use crate::{player::commons::Player, state::ArcPipe};
use controls::build_controls;
use gtk4::{prelude::BoxExt, ActionBar, Box, Orientation};
use subtitles::build_subtitles;
use time_bar::build_time_bar;
use volume::build_volume;

mod controls;
mod subtitles;
mod time_bar;
mod volume;

pub fn build_footer(state: ArcPipe<impl Player>) -> ActionBar {
    let footer = ActionBar::new();

    let container = Box::new(Orientation::Vertical, 3);

    let top = Box::new(Orientation::Horizontal, 0);
    top.append(&build_controls(state.clone()));
    top.append(&build_time_bar(state.clone()));
    top.append(&build_volume(state.clone()));

    let bottom = Box::new(Orientation::Horizontal, 0);
    bottom.append(&build_subtitles(state.clone()));

    container.append(&top);
    container.append(&bottom);

    footer.set_center_widget(Some(&container));

    footer
}
