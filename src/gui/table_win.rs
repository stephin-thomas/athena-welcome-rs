use adw::glib::clone;
use gtk::{gio, glib::BoxedAnyObject, prelude::*, Orientation};
use gtk::{Box, SingleSelection};
use serde::de::DeserializeOwned;

use std::cell::{Ref, RefCell};
use std::path::Path;

use crate::gui::gobjects;
use crate::utils::AsArray;

use adw::prelude::*;
use adw::ApplicationWindow;
use std::rc::Rc;

use super::APP_NAME;
use adw::gio::ActionEntry;

pub(crate) fn create<S, T>(
    app: &adw::Application,
    title: &str,
    header: &[&'static str],
    filter_index: usize,
    filter_dropdown: Option<Vec<String>>,
    csv_data: Rc<Vec<T>>,
    col_width: Option<&[i32]>,
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
        .modal(true)
        .build();
    let table_box = draw(csv_data, header, filter_index, filter_dropdown, col_width);
    window.set_content(Some(&table_box));

    let action_close = ActionEntry::builder("close")
        .activate(|window: &ApplicationWindow, _, _| {
            window.close();
        })
        .build();
    window.add_action_entries([action_close]);
    // window.set_parent(parent_win.as_ref());
    // window.set_destroy_with_parent(true);
    // window.modal(true);
    // Present window
    window.present();
}

pub(crate) fn draw<T>(
    csv_data: Rc<Vec<T>>,
    header: &[&'static str],
    filter_index: usize,
    filter_vals: Option<Vec<String>>,
    col_width: Option<&[i32]>,
) -> Box
where
    T: AsArray,
    T: Clone,
    T: 'static,
{
    let val_dropdown = Rc::new(RefCell::new("None".to_owned()));
    // Create columns and add them to the table
    let list_store = Rc::new(gen_list_store(Rc::clone(&csv_data)));
    let cust_filter = gtk::CustomFilter::new(clone!(@strong val_dropdown=> move |obj| {
        let record = obj.downcast_ref::<BoxedAnyObject>().unwrap();
        let r: Ref<T> = record.borrow();
        let val_dropdown_ref=val_dropdown.borrow();
        if r.as_array()[filter_index] == *val_dropdown_ref|| *val_dropdown_ref == "None".to_owned()  {
        // println!("{}=={}\n",val_dropdown_ref, r.as_array()[filter_index]);
            return true;
        } else {
        // println!("{}!={}\n",val_dropdown_ref, r.as_array()[filter_index]);
            return false;
        }
    }));
    let filter_store = gtk::FilterListModel::builder()
        .model(list_store.as_ref())
        .filter(&cust_filter.clone())
        .build();
    let sel = gtk::SingleSelection::builder().model(&filter_store).build();

    let columnview = gtk::ColumnView::new(Some(sel));
    if let Some(col_w) = col_width {
        if col_w.len() != header.len() {
            panic!("Header and Column widths don't match")
        }
    };
    for (i, header_text) in header.into_iter().enumerate() {
        let col_f = gtk::SignalListItemFactory::new();
        col_f.connect_setup(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let row = gtk::Label::builder()
                .justify(gtk::Justification::Left)
                .wrap(true)
                .halign(gtk::Align::Start)
                // .xalign(0.0)
                // .width_request(col_width)
                .build();
            item.set_child(Some(&row));
        });
        col_f.connect_bind(move |_factory, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();
            let r: Ref<T> = entry.borrow();
            let child = item.child().and_downcast::<gtk::Label>().unwrap();
            child.set_text(r.as_array()[i.clone()].as_str());
            child.set_justify(gtk::Justification::Left);
        });
        let col = gtk::ColumnViewColumn::new(Some(header_text), Some(col_f));
        if let Some(col_w) = col_width {
            col.set_fixed_width(col_w[i]);
        };
        columnview.append_column(&col)
    }
    let scrolled_window = gtk::ScrolledWindow::builder()
        // .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .build();

    scrolled_window.set_child(Some(&columnview));
    scrolled_window.set_vexpand(true);
    scrolled_window.set_hexpand(true);
    scrolled_window.set_min_content_height(250);
    let vbox = Box::builder()
        .orientation(Orientation::Vertical)
        // .homogeneous(true)
        .build();
    vbox.append(&adw::HeaderBar::new());
    vbox.append(&scrolled_window);

    let filter_dropdown: Option<gtk::DropDown> = match filter_vals {
        Some(filter_dropdown_vals) => {
            let drop_down_str: Vec<&str> =
                filter_dropdown_vals.iter().map(String::as_ref).collect();
            let drop_down = gtk::DropDown::from_strings(drop_down_str.as_slice());
            drop_down.set_size_request(200, 1);
            drop_down.set_halign(gtk::Align::End);
            drop_down.connect_selected_notify(clone!(@strong val_dropdown=> move |signal| {
                let sign = signal.selected();
                let val_dropdown_mut = filter_dropdown_vals
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
                    .unwrap()
                    .1
                    .to_owned();
                val_dropdown.replace(val_dropdown_mut);
            let model = columnview.model().unwrap();
            let single_select_model: SingleSelection = model.downcast().unwrap();
            let filter_store = gtk::FilterListModel::builder()
                .model(list_store.as_ref())
                .filter(&cust_filter.clone())
                .build();
            single_select_model.set_model(Some(&filter_store));
            }));

            Some(drop_down)
        }
        None => {
            println!("No DropDown");
            None
        }
    };

    let hbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .homogeneous(true)
        .build();
    if filter_dropdown.is_some() {
        let filter_lbl = gobjects::create_generic_label(gtk::Justification::Left);
        filter_lbl.set_label("Filter");
        hbox.append(&filter_lbl);
        hbox.append(filter_dropdown.as_ref().unwrap());
    }
    vbox.append(&hbox);

    vbox.set_css_classes(&["boxed-list"]);

    return vbox;
}

fn gen_list_store<T>(role_csv_data: Rc<Vec<T>>) -> gio::ListStore
where
    T: AsArray,
    T: Clone,
    T: 'static,
{
    let store = gio::ListStore::new::<BoxedAnyObject>();
    for record in role_csv_data.as_ref() {
        store.append(&BoxedAnyObject::new(record.clone()))
    }
    store
}
