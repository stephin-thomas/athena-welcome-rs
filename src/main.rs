use anyhow::{Context, Result};
mod csv_data;
use dirs;
use gtk::glib;
// use gtk::prelude::*;
use lazy_static::lazy_static;
use std::path::PathBuf;
use std::sync::OnceLock;
use tokio::runtime::Runtime;
mod gui;
mod settings;
mod utils;
use adw::prelude::*;
const APP_ID: &str = "org.athenaos.athena-welcome";
const APP_NAME: &str = "athena-welcome";

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}

fn get_app_config_dir() -> Result<PathBuf> {
    let mut usr_config_dir: PathBuf =
        dirs::config_dir().context("Could not find user config directory")?;
    usr_config_dir.push(APP_NAME);
    Ok(usr_config_dir)
}

lazy_static! {
    pub static ref APP_CONFIG_DIR: PathBuf = get_app_config_dir().unwrap();
    pub static ref ASSETS: PathBuf = PathBuf::from("./assets/");
    pub static ref SETTINGS_PATH: PathBuf = APP_CONFIG_DIR.join("settings.conf");
}

fn main() -> glib::ExitCode {
    settings::Config::init().unwrap();
    // Create a new application
    let application = adw::Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    application.connect_activate(gui::build_ui);
    application.run()

    // Run the application
}
