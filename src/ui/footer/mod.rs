use crate::{player::commons::Player, state::ArcPipe};
use controls::build_controls;
use gtk4::{prelude::BoxExt, ActionBar, Box, Orientation};
use time_bar::build_time_bar;
use volume::build_volume;

mod controls;
mod time_bar;
mod volume;

pub fn build_footer(state: ArcPipe<impl Player>) -> ActionBar {
    let footer = ActionBar::new();

    let container = Box::new(Orientation::Horizontal, 0);

    container.append(&build_controls(state.clone()));
    container.append(&build_time_bar(state.clone()));
    container.append(&build_volume(state.clone()));

    footer.set_center_widget(Some(&container));

    footer
}
