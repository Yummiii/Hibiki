use gtk4::prelude::ApplicationExtManual;
use streamer::init_pipeline;
use ui::create_ui;

mod streamer;
mod ui;

fn main() {
    env_logger::init();
    libadwaita::init().unwrap();
    gstreamer::init().unwrap();

    let test = init_pipeline();

    create_ui(test).run_with_args(&[""]);
    // let args = env::args().collect::<Vec<String>>();
}
