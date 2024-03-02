use gtk::{gio, glib::BoxedAnyObject, prelude::*, Orientation};
use gtk::{Box, SingleSelection};

use serde::de::DeserializeOwned;

use std::cell::Ref;
use std::path::Path;

use crate::gui::gobjects;
use crate::utils::read_csv_data;
use crate::utils::AsArray;
use crate::ASSETS;

use adw::prelude::*;
use adw::ApplicationWindow;
use std::rc::Rc;

use super::APP_NAME;
use adw::gio::ActionEntry;

pub(crate) fn create<S, T>(
    app: &adw::Application,
    title: &str,
    header: [&'static str; 3],
    filter_index: usize,
    filter_dropdown: Vec<String>,
    csv_path: S,
    col_width: Option<[i32; 3]>,
    parent_win: Rc<ApplicationWindow>,
) where
    S: AsRef<Path>,
    T: AsArray,
    T: DeserializeOwned,
    T: Clone,
    T: 'static,
    //for debugging purpose only
    T: std::fmt::Debug,
{
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title(title)
        .default_height(250)
        .default_width(920)
        .icon_name(APP_NAME)
        .build();
    window.set_transient_for(Some(parent_win.as_ref()));
    // let window = Rc::new(window);
    let mut csv_abs_path = ASSETS.clone();
    csv_abs_path.push(csv_path);
    let csv_data: Rc<Vec<T>> = Rc::new(read_csv_data(csv_abs_path));
    println!("Read csv file");
    let roles_win = draw(csv_data, header, filter_index, filter_dropdown, col_width);
    window.set_content(Some(&roles_win));

    let action_close = ActionEntry::builder("close")
        .activate(|window: &ApplicationWindow, _, _| {
            window.close();
        })
        .build();
    window.add_action_entries([action_close]);
    window.set_parent(parent_win.as_ref());
    window.set_destroy_with_parent(true);
    // Present window
    window.present();
}

pub(crate) fn draw<T>(
    csv_data: Rc<Vec<T>>,
    header: [&'static str; 3],
    filter_index: usize,
    filter_dropdown: Vec<String>,
    col_width: Option<[i32; 3]>,
) -> Box
where
    T: AsArray,
    T: Clone,
    T: 'static,
{
    // Create columns and add them to the table
    let store = filter_list(Rc::clone(&csv_data), "None".to_owned(), filter_index);

    let sel = gtk::SingleSelection::new(Some(store));
    let columnview = Rc::new(gtk::ColumnView::new(Some(sel)));
    let col1factory = gtk::SignalListItemFactory::new();
    let col2factory = gtk::SignalListItemFactory::new();
    let col3factory = gtk::SignalListItemFactory::new();
    col1factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        // let row = gtk::Label::new(None);
        let row = gtk::Label::builder()
            .justify(gtk::Justification::Left)
            .wrap(true)
            .xalign(0.0)
            // .width_request(col_width)
            .build();
        item.set_child(Some(&row));
    });
    col1factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<T> = entry.borrow();
        // if r.role == role1 {
        let child = item.child().and_downcast::<gtk::Label>().unwrap();
        // child.set_width_request(200_i32);
        child.set_text(r.as_array()[0].as_str());
        child.set_justify(gtk::Justification::Left);
        // }
    });
    // col1factory.connect("filter",true ,|role|{} )
    col2factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = gtk::Label::builder()
            .justify(gtk::Justification::Left)
            .wrap(true)
            .xalign(0.0)
            // .width_request(col_width)
            .build();

        item.set_child(Some(&row));
    });
    // col1factory.connect("filter",true ,move |role|{store2.borrow_mut().remove_all(); return None} )
    col2factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<T> = entry.borrow();
        // if r.role == role2 {
        let child = item.child().and_downcast::<gtk::Label>().unwrap();
        // child.set_width_request(200_i32);
        child.set_text(r.as_array()[1].as_str());
        child.set_justify(gtk::Justification::Left);
        // }
    });
    col3factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = gtk::Label::builder()
            .justify(gtk::Justification::Left)
            .wrap(true)
            .xalign(0.0)
            // .width_request(col_width)
            .build();
        item.set_child(Some(&row));
    });

    col3factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<T> = entry.borrow();
        // if r.role == role3 {
        let child = item.child().and_downcast::<gtk::Label>().unwrap();
        // child.set_width_request(200_i32);
        child.set_justify(gtk::Justification::Left);
        child.set_text(r.as_array()[2].as_str());
        // }
    });
    let col1 = gtk::ColumnViewColumn::new(Some(header[0]), Some(col1factory));
    let col2 = gtk::ColumnViewColumn::new(Some(header[1]), Some(col2factory));
    let col3 = gtk::ColumnViewColumn::new(Some(header[2]), Some(col3factory));
    if let Some(col_w) = col_width {
        col1.set_fixed_width(col_w[0]);
        col2.set_fixed_width(col_w[1]);
        col3.set_fixed_width(col_w[2]);
    }
    columnview.append_column(&col1);
    columnview.append_column(&col2);
    columnview.append_column(&col3);
    let column_view_rc = Rc::clone(&columnview);
    let scrolled_window = gtk::ScrolledWindow::builder()
        // .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .build();

    scrolled_window.set_child(Some(columnview.as_ref()));
    let drop_down_str: Vec<&str> = filter_dropdown.iter().map(String::as_ref).collect();
    let role_dropdown = gtk::DropDown::from_strings(drop_down_str.as_slice());
    role_dropdown.set_size_request(200, 1);
    role_dropdown.set_halign(gtk::Align::End);
    let filter_dropdown_1 = filter_dropdown.clone();
    role_dropdown.connect_selected_notify(move |signal| {
        let sign = signal.selected();
        let role = filter_dropdown_1
            .iter()
            .enumerate()
            .find(
                move |(index, _)| {
                    if *index == sign as usize {
                        true
                    } else {
                        false
                    }
                },
            )
            .unwrap();
        let model = column_view_rc.model().unwrap();
        let single_select_model: SingleSelection = model.downcast().unwrap();
        let store = filter_list(Rc::clone(&csv_data), (*role.1).to_owned(), filter_index);
        single_select_model.set_model(Some(&store));
        column_view_rc.set_model(Some(&single_select_model));
    });

    let hbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .homogeneous(true)
        .build();
    let filter_lbl = gobjects::create_generic_label(gtk::Justification::Left);
    filter_lbl.set_label("Filter");
    hbox.append(&filter_lbl);
    hbox.append(&role_dropdown);
    scrolled_window.set_vexpand(true);
    scrolled_window.set_min_content_height(250);
    let vbox = Box::builder()
        .orientation(Orientation::Vertical)
        // .homogeneous(true)
        .build();
    vbox.append(&scrolled_window);
    vbox.append(&hbox);

    return vbox;
}

fn filter_list<T>(
    role_csv_data: Rc<Vec<T>>,
    filter_str: String,
    filter_index: usize,
) -> gio::ListStore
where
    T: AsArray,
    T: Clone,
    T: 'static,
{
    let store = gio::ListStore::new::<BoxedAnyObject>();
    for record in role_csv_data.as_ref() {
        // println!("Role {},csv_data {}", role, record.role);
        if record.as_array()[filter_index] == filter_str || "None" == filter_str {
            store.append(&BoxedAnyObject::new(record.clone()))
        }
    }
    store
}
//close this window when the main window closes
