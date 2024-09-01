pub mod macros {
    macro_rules! make {
        ($element:expr) => {
            make!($element, $element)
        };

        ($element:expr, $name:expr) => {
            gstreamer::ElementFactory::make($element)
                .name($name)
                .build()
        };
    }

    pub(crate) use make;
}
