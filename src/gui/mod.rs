use super::APP_NAME;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

pub(crate) fn build_ui(app: &Application) {
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
