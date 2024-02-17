use adw::prelude::*;
use tokio::process::Command;

async fn check_internet() -> bool {
    match reqwest::get("https://www.bing.com").await {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn start_cmd(cmd: &str, args: &[&str]) -> Option<std::process::Output> {
    let cmd_owned = cmd.to_owned();
    let args_owned = args
        .into_iter()
        .map(|&val| val.to_owned())
        .collect::<Vec<String>>();

    let output = Command::new(&cmd_owned).args(&args_owned).output().await;
    match output {
        Ok(output_run) => {
            println!("command:- {cmd_owned} {args_owned:?}");
            if output_run.status.success() {
                println!(
                    "STDOUT :\n {:?}",
                    std::str::from_utf8(&output_run.stdout).unwrap(),
                );
            } else {
                println!(
                    "STDERR :\n {:?}",
                    std::str::from_utf8(&output_run.stderr).unwrap()
                );
            }

            Some(output_run)
        }
        Err(err) => {
            println!("Error running {cmd_owned} {args_owned:?}\n {}", err);
            None
        }
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
