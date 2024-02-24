use std::cell::RefCell;
use std::rc::Rc;

use super::APP_NAME;
use crate::settings::Config;
use adw::gio::ActionEntry;
use adw::prelude::*;
use adw::ApplicationWindow;
mod gobjects;
mod logic;
mod role_tools_win;
pub mod welcome_win;

pub(crate) fn build_ui(app: &adw::Application) {
    let configs = Rc::new(RefCell::new(
        Config::load().expect("Error parsing settings file"),
    ));
    // Create a window and set the title

    let toast = Rc::new(adw::ToastOverlay::new());
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Athena Welcome")
        .default_height(250)
        .default_width(920)
        .icon_name(APP_NAME)
        .build();
    let window_rc = Rc::new(window);
    let welcome_win =
        welcome_win::draw(configs.clone(), window_rc.clone(), toast.clone(), app).unwrap();
    toast.set_child(Some(&welcome_win));
    window_rc.set_content(Some(toast.as_ref()));

    let action_close = ActionEntry::builder("close")
        .activate(|window: &ApplicationWindow, _, _| {
            window.close();
        })
        .build();
    window_rc.add_action_entries([action_close]);

    // Present window
    window_rc.present();
}
