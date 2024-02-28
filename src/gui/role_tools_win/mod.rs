use gtk::{gio, glib::BoxedAnyObject, prelude::*, Orientation};
use gtk::{Box, SingleSelection};

use strum::IntoEnumIterator;

use std::cell::Ref;

use crate::gui::gobjects;
use crate::{
    settings,
    utils::{read_csv_data, Record},
    ASSETS,
};

use crate::settings::Config;
use adw::prelude::*;
use adw::ApplicationWindow;
use std::cell::RefCell;
use std::rc::Rc;

use super::APP_NAME;
use adw::gio::ActionEntry;

pub(crate) fn create(app: &adw::Application, configs: Rc<RefCell<Config>>) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Athena Roles")
        .default_height(250)
        .default_width(920)
        .icon_name(APP_NAME)
        .build();
    // let window = Rc::new(window);
    let role = &configs.borrow().role;
    let roles_win = draw(role.name().to_owned());
    window.set_content(Some(&roles_win));

    let action_close = ActionEntry::builder("close")
        .activate(|window: &ApplicationWindow, _, _| {
            window.close();
        })
        .build();
    window.add_action_entries([action_close]);
    // Present window
    window.present();
}

pub(crate) fn draw(role: String) -> Box {
    println!("Filter {}", role);
    let mut csv_path = ASSETS.clone();
    csv_path.push("roles.csv");
    let role_csv_data = Rc::new(read_csv_data(csv_path));
    println!("Read csv file");
    // Create columns and add them to the table
    let store = filter_list(Rc::clone(&role_csv_data).as_ref(), "None".to_owned());

    let sel = gtk::SingleSelection::new(Some(store));
    let columnview = Rc::new(gtk::ColumnView::new(Some(sel)));

    let col1factory = gtk::SignalListItemFactory::new();
    let col2factory = gtk::SignalListItemFactory::new();
    let col3factory = gtk::SignalListItemFactory::new();
    col1factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = gtk::Label::new(None);
        item.set_child(Some(&row));
    });
    col1factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<Record> = entry.borrow();
        // if r.role == role1 {
        let child = item.child().and_downcast::<gtk::Label>().unwrap();
        // child.set_width_request(200_i32);
        child.set_text(r.role.as_str());
        child.set_justify(gtk::Justification::Left);
        // }
    });
    // col1factory.connect("filter",true ,|role|{} )
    col2factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = gtk::Label::new(None);
        item.set_child(Some(&row));
    });
    // col1factory.connect("filter",true ,move |role|{store2.borrow_mut().remove_all(); return None} )
    col2factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<Record> = entry.borrow();
        // if r.role == role2 {
        let child = item.child().and_downcast::<gtk::Label>().unwrap();
        // child.set_width_request(200_i32);
        child.set_text(r.tool.as_str());
        child.set_justify(gtk::Justification::Left);
        // }
    });
    col3factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = gtk::Label::new(None);
        item.set_child(Some(&row));
    });

    col3factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
        let r: Ref<Record> = entry.borrow();
        // if r.role == role3 {
        let child = item.child().and_downcast::<gtk::Label>().unwrap();
        // child.set_width_request(200_i32);
        child.set_justify(gtk::Justification::Left);
        child.set_text(r.desc.as_str());
        // }
    });
    let role_col = gtk::ColumnViewColumn::new(Some("Role"), Some(col1factory));
    let tool_col = gtk::ColumnViewColumn::new(Some("Tool"), Some(col2factory));
    let desc_col = gtk::ColumnViewColumn::new(Some("Description"), Some(col3factory));
    columnview.append_column(&role_col);
    columnview.append_column(&tool_col);
    columnview.append_column(&desc_col);
    let column_view_rc = Rc::clone(&columnview);
    let scrolled_window = gtk::ScrolledWindow::builder()
        // .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .build();

    scrolled_window.set_child(Some(columnview.as_ref()));

    let roles_string: Vec<String> = settings::Role::iter()
        .map(|role| role.to_string())
        .collect();
    let roles: Vec<&str> = roles_string.iter().map(String::as_ref).collect();
    let role_dropdown = gtk::DropDown::from_strings(roles.as_slice());
    role_dropdown.set_size_request(200, 1);
    role_dropdown.set_halign(gtk::Align::End);
    role_dropdown.connect_selected_notify(move |signal| {
        let sign = signal.selected();
        let role = settings::Role::iter()
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
        let store = filter_list(Rc::clone(&role_csv_data).as_ref(), role.1.name().to_owned());
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

fn filter_list(role_csv_data: &Vec<Record>, role: String) -> gio::ListStore {
    let store = gio::ListStore::new::<BoxedAnyObject>();
    for record in role_csv_data {
        // println!("Role {},csv_data {}", role, record.role);
        if record.role == role || "None" == role {
            store.append(&BoxedAnyObject::new(record.clone()))
        }
    }
    store
}

// fn filter_store(store: &gio::ListStore, role: String) {
//     store.retain(|item| {
//         let item = item.downcast_ref::<BoxedAnyObject>().unwrap();
//         // let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
//         let r: Ref<Record> = item.borrow();
//         if r.role == role || role == "None" {
//             return true;
//         } else {
//             return false;
//         }
//     });
// }
