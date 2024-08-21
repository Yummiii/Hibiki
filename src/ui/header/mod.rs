use crate::streamer::ArcPipe;
use libadwaita::HeaderBar;
use menu::build_menu;
use picker::build_picker;

mod menu;
mod picker;

pub fn build_header(pipeline: ArcPipe) -> HeaderBar {
    let header = HeaderBar::new();

    header.pack_start(&build_picker(pipeline));
    header.pack_end(&build_menu());

    header
}
