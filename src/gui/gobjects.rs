use gtk::glib;
use gtk::prelude::*;

pub(crate) fn btn_n_ttp_label(
    text: &str,
    ttp_markup: Option<&str>,
    width_req: i32,
    height_req: i32,
) -> gtk::Button {
    let btn = gtk::Button::with_label(text);
    btn.set_tooltip_markup(ttp_markup);
    btn.set_size_request(width_req, height_req);
    btn
}

pub(crate) fn gen_img_btn(
    file: impl Into<glib::GString>,
    lbl_str: &str,
    width_req: i32,
    height_req: i32,
) -> gtk::Button {
    let img = gtk::Image::builder()
        .file(file)
        .pixel_size(height_req - 20)
        .halign(gtk::Align::End)
        .build();
    let lbl = gtk::Label::new(None);
    lbl.set_halign(gtk::Align::Center);
    lbl.set_markup(lbl_str);
    let grd = gtk::Grid::builder()
        // .column_homogeneous(true)
        .row_homogeneous(true)
        .build();

    grd.attach(&img, 0, 0, 1, 1);
    grd.attach_next_to(&lbl, Some(&img), gtk::PositionType::Right, 1, 1);
    grd.set_halign(gtk::Align::Center);
    grd.set_column_spacing(5);
    let btn = gtk::Button::builder()
        // .halign(gtk::Align::Center)
        // .valign(gtk::Align::Center)
        .child(&grd)
        .halign(gtk::Align::Center)
        .build();
    btn.set_size_request(width_req, height_req);
    btn.set_halign(gtk::Align::Center);
    btn
}

pub(crate) fn create_btn(width: i32, height: i32, markup: &str) -> gtk::Button {
    let btn = gtk::Button::builder()
        .width_request(width)
        .height_request(height)
        .build();
    let lbl = gtk::Label::new(None);
    lbl.set_markup(markup);
    btn.set_child(Some(&lbl));
    btn.set_halign(gtk::Align::Center);
    btn
}

pub(crate) fn create_generic_label(justification: gtk::Justification) -> gtk::Label {
    gtk::Label::builder()
        .xalign(0_f32)
        .justify(justification)
        .wrap(true)
        .build()
}
