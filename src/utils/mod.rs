use std::ffi::OsStr;

use tokio::process::Command;

pub async fn internet_connected() -> bool {
    match isahc::get_async("https://www.bing.com").await {
        Ok(_) => true,
        Err(err) => {
            println!("Error occured while checking internet {:?}", err);
            false
        }
    }
}

pub async fn start_cmd<I, S>(cmd: &str, args: I) -> Option<std::process::Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let cmd_owned = cmd.to_owned();
    // let args_owned = args
    //     .into_iter()
    //     .map(|&val| val.to_owned())
    //     .collect::<Vec<String>>();

    let output = Command::new(&cmd_owned).args(args).output().await;
    match output {
        Ok(output_run) => {
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
            println!("Error running {cmd_owned}\n {}", err);
            None
        }
    }
}
