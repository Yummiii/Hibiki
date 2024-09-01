use gtk4::{gio::resources_register_include, prelude::ApplicationExtManual};
use log::debug;
use player::{backends::gstreamer::GStreamerPlayer, commons::media_controls::MediaControls};
use state::AppState;
use std::{env, path::Path};
use ui::create_ui;
use url::Url;

mod messenger;
mod player;
mod state;
mod ui;

fn main() {
    env_logger::init();

    resources_register_include!("compiled.gresource").unwrap();

    libadwaita::init().unwrap();
    gstreamer::init().unwrap();

    let args = env::args().collect::<Vec<String>>();
    let state = AppState::new::<GStreamerPlayer>();

    if args.len() > 1 {
        let path = Path::new(args[1].as_str());
        let uri = if path.exists() {
            let uri = Url::from_file_path(path).unwrap().to_string();
            debug!("File: {}", uri);
            uri
        } else {
            args[1].clone()
        };

        state.player.play(&uri);
    }

    create_ui(state).run_with_args(&[""]);
}
