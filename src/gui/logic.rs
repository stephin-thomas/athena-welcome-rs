use adw::prelude::*;
use async_channel::Sender;
use std::process::Output;
use whoami;

pub(crate) fn is_live_user() -> bool {
    if whoami::username() == "liveuser" {
        println!("user is liveuser");
        true
    } else {
        false
    }
}
pub(crate) fn get_startup_text() -> String {
    if is_live_user() {
        String::from("During the installation many options will be open to you. You have the freedom of choice.\n\
    We communicate with our community via a diversity of social media.\n\
    Join us to learn the latest news, ask questions or for casual talk.\n\
    Reach us on <b>Discord</b> for chatting or assistance.\n\
    ")
    } else {
        String::from("Choose your role and click the <b>Set Your Role</b> button to retrieve the main resources you need!\n\n\
    Click <b>HTB Update</b> to set your Hack The Box API key and start your hacking experience!\n\n\
    Get started on Athena. We communicate with our community
through Discord or GitHub.\n\
        Join us to learn the latest news, ask questions or just for chatting.\n\
    Open a <b>ticket</b> for any issues or proposals.\n\
Learn, study and have fun!")
    }
}
pub(crate) fn get_widget_by_name(hbox_vec: &Vec<gtk::Box>, name: &str) -> Option<gtk::Widget> {
    for hbx in hbox_vec.iter() {
        let mut child = hbx.first_child();
        while child.is_some() {
            if child.as_ref().unwrap().widget_name() == name {
                return child;
            } else {
                child = child.unwrap().next_sibling();
                continue;
            }
        }
    }
    None
}
pub(crate) async fn process_click(
    res: Option<Output>,
    toast_sen: Sender<String>,
    btn_dis_send: Sender<String>,
    btn_id: String,
) {
    if res.as_ref().is_some() {
        let result = res.unwrap();
        if result.status.success() {
            toast_sen
                .send("Task successfully completed".to_owned())
                .await
                .expect("Error opening channel");
        } else {
            toast_sen
                .send(format!("Task failed with error code {}", result.status))
                .await
                .expect("Error opening channel");
        }
    } else {
        toast_sen
            .send("Error make sure all the dependencies installed".to_owned())
            .await
            .expect("Error opening channel");
    }
    //Remove the following line important. Only for testing
    // sleep(Duration::from_millis(3000)).await;
    btn_dis_send
        .send(btn_id)
        .await
        .expect("Error sending through channel");
}
