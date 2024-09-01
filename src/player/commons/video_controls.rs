use gtk4::Picture;

pub trait VideoControls {
    fn set_widget(&self, widget: &Picture);
}
