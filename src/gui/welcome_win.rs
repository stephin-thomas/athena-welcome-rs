use super::gobjects;
use super::logic::{get_startup_text, get_widget_by_name, is_live_user, process_click};
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
    //Channel to send signal to make a button sensitive after executing a command.
    let (btn_dis_send, btn_dis_receiver) = async_channel::bounded(1);
    //Messages send through will be shown as toasts.
    let (toast_sen, toast_rec) = async_channel::bounded(1);
    let open_url = clone!(@strong toast_sen=> move |url:&'static str| {
        if let Err(err) = open::that(url) {
            runtime().spawn(clone!(@strong toast_sen => async move {
            let err_msg=format!("error opening browser :- {}",err);
            toast_sen.send(err_msg).await.expect("error opening channels");
            }));
        };
    });
    let cmd_on_click = clone!(@strong toast,@strong toast_sen, @strong btn_dis_send =>move |btn:&gtk::Button,widget_name:&str,cmd:&'static str,args:&'static [&'static str]| {
            btn.set_sensitive(false);
            btn.set_widget_name(widget_name);
            let btn_id= btn.widget_name().to_string();

            runtime().spawn(clone!(@strong toast_sen, @strong btn_dis_send =>async move {
                let response = start_cmd(cmd, args ).await;
                process_click(response,toast_sen ,btn_dis_send , btn_id).await;
                            }));});
    let cmd_on_click_owned = clone!(@strong toast,@strong toast_sen, @strong btn_dis_send =>move |btn:&gtk::Button,widget_name:&str,cmd:&'static str, args:Vec<String>| {
            btn.set_sensitive(false);
            btn.set_widget_name(widget_name);
            let btn_id= btn.widget_name().to_string();
            // let args_vec = args.into_iter().map(|str_val|str_val).collect::<Vec<&str>>();
            runtime().spawn(clone!(@strong toast_sen, @strong btn_dis_send =>async move {
                let response = start_cmd(cmd, args.as_slice() ).await;
                process_click(response,toast_sen ,btn_dis_send , btn_id).await;
                            }));});
    //Parent Box holding all the widgets.
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
    label_2.set_markup(get_startup_text().as_str());
    hbox_vec[1].set_halign(gtk::Align::Center);
    hbox_vec[1].append(&label_2);

    let btn_channels = gobjects::create_btn(
        300,
        70,
        "<span size='large'><b>Update Nix channels</b></span>",
    );

    btn_channels.connect_clicked(clone!(@strong cmd_on_click =>move |btn|
    cmd_on_click(btn, "nix_channels", "pkexec", &["nix-channel", "--update"]);
    ));

    let btn_opn_tkt = gobjects::btn_n_ttp_label("Open a ticket", None, 200, 50);
    let btn_os_info = gobjects::btn_n_ttp_label("Athena OS project", None, 200, 50);
    let btn_rel_info = gobjects::btn_n_ttp_label("Release info", None, 200, 50);
    hbox_vec[5].append(&btn_rel_info);
    hbox_vec[5].append(&btn_os_info);
    hbox_vec[5].append(&btn_opn_tkt);
    hbox_vec[5].set_spacing(10);
    let btn_discord = gobjects::btn_n_ttp_label("Discord", None, 200, 50);
    let btn_demo = gobjects::btn_n_ttp_label("Video Demo", None, 200, 50);
    let btn_wiki = gobjects::btn_n_ttp_label("Wiki", None, 200, 50);
    btn_rel_info.connect_clicked(clone!(@strong open_url=>move |_| {
        open_url("https://github.com/Athena-OS/athena-iso/releases");
    }));
    btn_os_info.connect_clicked(clone!(@strong open_url=>move |_| {
        open_url("https://github.com/Athena-OS");
    }));
    btn_opn_tkt.connect_clicked(clone!(@strong open_url=>move |_| {
        open_url("https://github.com/Athena-OS/athena-iso/issues/new/choose");
    }));
    btn_discord.connect_clicked(clone!(@strong open_url=>move |_| {
        open_url("https://discord.gg/athena-os-977645785170714644");
    }));
    btn_demo.connect_clicked(clone!(@strong open_url=>move |_| {
        open_url("https://www.youtube.com/watch?v=4_ZY9Tj4U_8");
    }));
    btn_wiki.connect_clicked(clone!(@strong open_url=>move |_| {
        open_url("https://github.com/Athena-OS/athena-iso/wiki");
    }));
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
    if !is_live_user() {
        //Drop box
        let roles_string: Vec<String> = settings::Role::iter()
            .map(|role| role.to_string())
            .collect();
        let roles: Vec<&str> = roles_string.iter().map(String::as_ref).collect();
        let role_dropdown = gtk::DropDown::from_strings(roles.as_slice());
        role_dropdown.set_size_request(200, 1);
        let pos = roles
            .iter()
            .position(|&v| {
                let cur_role = &configs.borrow().role;
                v == cur_role.to_string()
            })
            .context("error getting roles")?;
        role_dropdown.set_selected(pos as u32);
        role_dropdown.set_halign(gtk::Align::End);
        // let configs_rc_cl = Rc::clone(&configs_rc);
        role_dropdown.connect_selected_notify(clone!(@strong configs =>move |signal| {
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
                configs.borrow_mut().role = role.1;
            }
                configs.borrow().save().unwrap();
        }));

        let btn_htb = gobjects::gen_img_btn(
            "assets/htb.png",
            "<span size='large'><b>HTB Update</b></span>",
            300,
            50,
        );
        btn_htb.connect_clicked(clone!(@strong cmd_on_click =>move |btn|
            cmd_on_click(btn, "htb_upd", "shell-rocket", &["htb-toolkit -u"]);
        ));

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
        let rtm_cmd: String = format!("sudo cyber-toolkit {:?}", configs.borrow().role);
        let btn_rtm =
            gobjects::create_btn(300, 70, "<span size='large'><b>Set Your Role</b></span>");
        btn_rtm.connect_clicked(clone!(@strong cmd_on_click_owned=>move |btn|{
            cmd_on_click_owned(btn, "btn_rtm", "shell-rocket", vec![rtm_cmd.clone()]);}
        ));

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
        btn_upgrade.connect_clicked(clone!(@strong cmd_on_click =>move |btn|
            cmd_on_click(btn, "upgrade", "shell-rocket", &["sudo nix-channel --update; sudo nixos-rebuild switch"]);
        ));

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
        let btn_gparted =
            gobjects::create_btn(300, 70, "<span size='large'><b>Run GParted</b></span>");
        btn_gparted.connect_clicked(clone!(@strong cmd_on_click =>move |btn|
        cmd_on_click(btn, "gparted", "gparted", &[]);
        ));
        let btn_non_linstall = gobjects::create_btn(
            300,
            70,
            "<span size='large'><b>Installation (Online)</b></span>",
        );
        btn_non_linstall.set_halign(Align::Center);

        btn_non_linstall.connect_clicked(clone!(@strong cmd_on_click =>move |btn|
        cmd_on_click(btn, "non_linstall", "/usr/bin/aegis-gui", &[]);
        ));

        hbox_vec[3].append(&btn_gparted);
        hbox_vec[2].append(&btn_non_linstall);
    }

    hbox_vec[3].append(&btn_channels);
    let auto_start_checkbox = gtk::CheckButton::with_label("Autostart");
    auto_start_checkbox.set_active(configs.borrow().autostart);
    auto_start_checkbox.connect_toggled(clone!(@strong configs  =>move |check_btn| {
        configs.borrow_mut().autostart = check_btn.is_active();
        configs.borrow().save().unwrap();
    }));
    auto_start_checkbox.set_halign(gtk::Align::End);

    hbox_vec[7].append(&label_warning);
    hbox_vec[7].append(&auto_start_checkbox);

    for box_v in hbox_vec.iter() {
        vbox.append(box_v);
    }

    // The main loop executes the asynchronous block
    glib::spawn_future_local(async move {
        loop {
            if let Ok(widget_name) = btn_dis_receiver.recv().await {
                if let Some(btn) = get_widget_by_name(&hbox_vec, widget_name.as_str()) {
                    btn.set_sensitive(true);
                };
            }
            if let Ok(msg) = toast_rec.recv().await {
                toast.add_toast(adw::Toast::new(msg.as_str()));
            }
        }
    });

    Ok(vbox)
}
