use super::{gobjects, logic};
use crate::runtime;
use crate::settings;
use crate::settings::Config;
use crate::utils::start_cmd;
use adw::glib::clone;
use adw::prelude::*;
use adw::ApplicationWindow;
use anyhow::{Context, Result};
use gtk::glib;
use gtk::Align;
use gtk::{Box, Orientation};
use std::cell::RefCell;
use std::rc::Rc;
use strum::IntoEnumIterator;
pub fn draw(
    configs: Rc<RefCell<Config>>,
    window: Rc<ApplicationWindow>,
    toast: Rc<adw::ToastOverlay>,
) -> Result<Box> {
    let (sender, receiver) = async_channel::bounded(1);
    let vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .margin_start(10)
        .margin_bottom(10)
        .margin_top(10)
        .margin_end(10)
        .build();
    let mut hbox_vec: Vec<gtk::Box> = Vec::with_capacity(10);
    for _ in 0..11 {
        let hbox = Box::builder()
            .orientation(Orientation::Horizontal)
            .homogeneous(true)
            .build();
        hbox_vec.push(hbox);
    }
    //Infobox
    let infobutton = gtk::Button::new();
    infobutton.set_has_tooltip(true);
    let image = gtk::Image::builder()
        .file("assets/question.png")
        .pixel_size(30)
        .build();
    infobutton.set_child(Some(&image));

    //Welcome Label
    let welcome_msg = gobjects::create_generic_label(gtk::Justification::Left);
    welcome_msg.set_markup("<big>Welcome to <b>Athena OS</b></big>");
    hbox_vec[0].append(&welcome_msg);

    //Label2
    let label_2 = gobjects::create_generic_label(gtk::Justification::Center);
    label_2.set_markup(logic::get_startup_text().as_str());
    hbox_vec[1].set_halign(gtk::Align::Center);
    hbox_vec[1].append(&label_2);

    let btn_channels = gobjects::create_btn(
        300,
        70,
        "<span size='large'><b>Update Nix channels</b></span>",
    );
    btn_channels.connect_clicked(clone!(@strong toast =>move |_| {
        toast.add_toast(adw::Toast::new("Updating nix channels"));
        println!("Toast clicked")
    }));

    let btn_rel_info = gobjects::btn_n_ttp_label("Release info", None, 200, 50);

    let btn_os_info = gobjects::btn_n_ttp_label("Athena OS project", None, 200, 50);
    let btn_opn_tkt = gobjects::btn_n_ttp_label("Open a ticket", None, 200, 50);

    hbox_vec[5].append(&btn_rel_info);
    hbox_vec[5].append(&btn_os_info);
    hbox_vec[5].append(&btn_opn_tkt);
    hbox_vec[5].set_spacing(10);
    let btn_discord = gobjects::btn_n_ttp_label("Discord", None, 200, 50);
    let btn_demo = gobjects::btn_n_ttp_label("Video Demo", None, 200, 50);
    let btn_wiki = gobjects::btn_n_ttp_label("Wiki", None, 200, 50);
    let btn_quit = gobjects::btn_n_ttp_label("Quit", None, 200, 50);

    btn_quit.connect_clicked(clone!(@strong window=>move |_| window.close()));
    let btn_donate = gobjects::create_btn(200, 20, "<b>Donate</b>");
    btn_quit.set_css_classes(&["destructive-action"]);
    hbox_vec[6].append(&btn_discord);
    hbox_vec[6].append(&btn_demo);
    hbox_vec[6].append(&btn_donate);
    hbox_vec[6].append(&btn_wiki);
    hbox_vec[6].append(&btn_quit);
    hbox_vec[6].set_spacing(10);
    //label warning
    let label_warning = gobjects::create_generic_label(gtk::Justification::Center);
    if !logic::is_live_user() {
        //Drop box
        let roles_string: Vec<String> = settings::Role::iter()
            .map(|role| role.to_string())
            .collect();
        let roles: Vec<&str> = roles_string.iter().map(String::as_ref).collect();
        let role_dropdown = gtk::DropDown::from_strings(roles.as_slice());
        role_dropdown.set_size_request(200, 1);
        let configs_rc = Rc::clone(&configs);
        let pos = roles
            .iter()
            .position(|&v| {
                let cur_role = &configs_rc.borrow().role;
                v == cur_role.to_string()
            })
            .context("error getting roles")?;
        role_dropdown.set_selected(pos as u32);
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
            {
                configs_rc.borrow_mut().role = role.1;
            }
            println!("Signal Called {:?}", configs_rc.borrow());
        });

        let btn_htb = gobjects::gen_img_btn(
            "assets/htb.png",
            "<span size='large'><b>HTB Update</b></span>",
            300,
            50,
        );

        let btn_tool = gobjects::gen_img_btn(
            "assets/tools_recipe.png",
            "<span size='large'><b>Tool Recipe</b></span>",
            300,
            50,
        );
        hbox_vec[0].append(&role_dropdown);
        hbox_vec[2].append(&btn_htb);
        hbox_vec[2].set_valign(gtk::Align::Center);
        hbox_vec[2].set_margin_top(10);
        hbox_vec[2].append(&btn_tool);
        let btn_rtm =
            gobjects::create_btn(300, 70, "<span size='large'><b>Set Your Role</b></span>");
        hbox_vec[3].append(&btn_rtm);
        let btn_role_tools = gobjects::btn_n_ttp_label(
            "Show Tools for Roles",
            Some("Show all the tools for each role"),
            300,
            0,
        );

        let btn_upgrade =
            gobjects::btn_n_ttp_label("Upgrade Athena", Some("Upgrade Athena"), 300, 0);

        // Connect to "clicked" signal of `button`
        btn_upgrade.connect_clicked(clone!(@strong toast=>move |_| {
            toast.add_toast(adw::Toast::new("Updating nix channels"));
            runtime().spawn(clone!(@strong sender => async move {
                let response = start_cmd("shell-rocket", &["sudo nix-channel --update; sudo nixos-rebuild switch"] ).await;
                sender.send(response).await.expect("The channel needs to be open.");
            }));
        }));

        let btn_hacking_var = gobjects::btn_n_ttp_label(
            "Hacking Variables",
            Some("Show the hacking variables"),
            300,
            0,
        );

        hbox_vec[4].append(&btn_role_tools);
        hbox_vec[4].set_spacing(10);
        hbox_vec[4].append(&btn_upgrade);
        hbox_vec[4].append(&btn_hacking_var);
    } else {
        // let grd = gtk::Grid::builder()
        //     .column_homogeneous(true)
        //     .row_homogeneous(true)
        //     .build();

        let btn_gparted =
            gobjects::create_btn(300, 70, "<span size='large'><b>Run GParted</b></span>");
        let btn_non_linstall = gobjects::create_btn(
            300,
            70,
            "<span size='large'><b>Installation (Online)</b></span>",
        );
        btn_non_linstall.set_halign(Align::Center);
        hbox_vec[3].append(&btn_gparted);
        hbox_vec[2].append(&btn_non_linstall);
    }

    hbox_vec[3].append(&btn_channels);
    let cl_configs = Rc::clone(&configs);
    let auto_start_checkbox = gtk::CheckButton::with_label("Autostart");
    auto_start_checkbox.set_active(cl_configs.borrow().autostart);
    auto_start_checkbox.connect_toggled(move |check_btn| {
        cl_configs.borrow_mut().autostart = check_btn.is_active();
        println!("Signal Called {:?}", cl_configs.borrow());
        {
            cl_configs.borrow().save().unwrap();
        };
    });
    auto_start_checkbox.set_halign(gtk::Align::End);

    hbox_vec[7].append(&label_warning);
    hbox_vec[7].append(&auto_start_checkbox);

    for box_v in hbox_vec.iter() {
        vbox.append(box_v);
    }

    // The main loop executes the asynchronous block
    glib::spawn_future_local(async move {
        while let Ok(success) = receiver.recv().await {
            let toast_msg = format!(
                "Operation :- {:?}",
                { || if success { "Success" } else { "Failed" } }()
            );
            toast.add_toast(adw::Toast::new(&toast_msg));
        }
    });

    Ok(vbox)
}
