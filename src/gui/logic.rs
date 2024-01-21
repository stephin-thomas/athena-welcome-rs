use whoami;

pub(crate) fn is_live_user() -> bool {
    if whoami::username() == "liveuser" {
        println!("user is liveuser");
        true
    } else {
        false
    }
}
