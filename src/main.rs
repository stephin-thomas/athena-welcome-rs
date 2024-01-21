use anyhow::{Context, Result};
use dirs;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use lazy_static::lazy_static;
use std::path::PathBuf;
use whoami;
mod configs;
const APP_ID: &'static str = "org.athenaos.athena-welcome";
const APP_NAME: &'static str = "athena-welcome";

fn get_app_config_dir() -> Result<PathBuf> {
    let mut usr_config_dir: PathBuf =
        dirs::config_dir().context("Could not find user config directory")?;
    usr_config_dir.push(&APP_NAME);
    return Ok(usr_config_dir);
}

lazy_static! {
    pub static ref APP_CONFIG_DIR: PathBuf = get_app_config_dir().unwrap();
    pub static ref SETTINGS_PATH: PathBuf = APP_CONFIG_DIR.join("settings.conf");
}

fn main() -> glib::ExitCode {
    configs::init_configs().unwrap();
    if whoami::username() == "liveuser" {
        println!("user is liveuser");
    }
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Athena Welcome")
        .default_height(250)
        .default_width(920)
        .icon_name(APP_NAME)
        .build();

    // Present window
    window.present();
}
