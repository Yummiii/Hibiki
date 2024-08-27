use gtk4::prelude::ApplicationExtManual;
use log::debug;
use std::{env, path::Path};
use streamer::init_pipeline;
use ui::create_ui;
use url::Url;

mod streamer;
mod ui;
mod messenger;

fn main() {
    env_logger::init();
    libadwaita::init().unwrap();
    gstreamer::init().unwrap();

    let args = env::args().collect::<Vec<String>>();

    let pipeline = init_pipeline();
    if args.len() > 1 {
        let uri = args[1].as_str();
        let path = Path::new(uri);
        if path.exists() {
            let uri = Url::from_file_path(path).unwrap().to_string();
            debug!("File: {}", uri);
            pipeline.play(&uri);
        } else {
            pipeline.play(&args[1]);
        }
    }

    create_ui(pipeline).run_with_args(&[""]);
}
