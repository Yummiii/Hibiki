use libadwaita::HeaderBar;
use menu::build_menu;

mod menu;

pub fn build_header() -> HeaderBar {
    let header = HeaderBar::new();

    header.pack_end(&build_menu());

    header
}
