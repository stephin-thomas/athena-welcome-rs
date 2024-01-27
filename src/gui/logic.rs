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
