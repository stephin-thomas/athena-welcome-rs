use std::{ffi::OsStr, fs::File, path::Path};
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

use csv::ReaderBuilder;
#[derive(Debug, serde::Deserialize, Clone)]
pub struct Record {
    pub role: String,
    pub tool: String,
    pub desc: String,
}
pub fn read_csv_data(path: impl AsRef<Path>) -> Vec<Record> {
    let mut csv_reader: csv::Reader<File> = ReaderBuilder::new()
        .delimiter(b';')
        .from_path(path)
        .expect("Error reading csv file");
    let rec_iter = csv_reader.deserialize();
    let records: Vec<Record> = rec_iter
        .filter(|rec| rec.is_ok())
        .map(|rec| rec.unwrap())
        .collect();
    return records;
}
