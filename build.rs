use std::{env, fs};
use xdg::BaseDirectories;

const APP_ID: &str = "com.zuraaa.Hibiki";

fn main() {
    glib_build_tools::compile_resources(
        &["resources"],
        "resources/resources.gresource.xml",
        "compiled.gresource",
    );

    #[cfg(target_os = "linux")]
    {
        if option_env!("CREATE_FILES").unwrap_or("true") == "true" {
            let dirs = BaseDirectories::with_prefix(APP_ID).unwrap();
            let data = dirs.get_data_home();
            fs::create_dir_all(&data).unwrap();

            //should I follow the highres icon spec?
            //that's a problem for future me
            let icon_path = data.join("icon.svg");
            // fs::copy("/home/yummi/Downloads/aaa.svg", &icon_path).unwrap();

            let profile = env::var("PROFILE").unwrap();
            let template = include_str!("resources/hibiki.desktop");

            let template = template.replace(
                "{{name}}",
                if profile == "release" {
                    "Hibiki"
                } else {
                    "Hibiki-dev"
                },
            );
            let template = template.replace("{{icon}}", &icon_path.display().to_string());

            let applications = BaseDirectories::with_prefix("applications")
                .unwrap()
                .get_data_home();

            fs::write(applications.join(format!("{}.desktop", APP_ID)), template).unwrap();

            println!("cargo:rerun-if-changed=resources/hibiki.desktop");
        }
    }
}
