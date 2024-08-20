use gtk4::prelude::ApplicationExtManual;
use ui::create_ui;

mod ui;

fn main() {
    env_logger::init();
    // libadwaita::init().unwrap();

    create_ui().run_with_args(&[""]);
    // let args = env::args().collect::<Vec<String>>();
}
