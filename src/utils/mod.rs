use tokio::process::Command;

async fn check_internet() -> bool {
    match reqwest::get("https://www.google.com").await {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn start_cmd(cmd: &str, args: &[&str]) -> bool {
    let cmd_owned = cmd.to_owned();
    let args_owned = args
        .into_iter()
        .map(|&val| val.to_owned())
        .collect::<Vec<String>>();

    let output = Command::new(cmd_owned)
        .args(args_owned)
        .output()
        .await
        .unwrap();
    println!(
        "STDOUT :\n {:?}",
        std::str::from_utf8(&output.stdout).unwrap(),
    );
    println!(
        "STDERR :\n {:?}",
        std::str::from_utf8(&output.stderr).unwrap()
    );

    output.status.success()
}
