use crate::{player::commons::Player, state::ArcPipe};
use libadwaita::HeaderBar;
use menu::build_menu;
use picker::build_picker;

mod menu;
mod picker;

pub fn build_header(state: ArcPipe<impl Player>) -> HeaderBar {
    let header = HeaderBar::new();

    header.pack_start(&build_picker(state));
    header.pack_end(&build_menu());

    header
}
