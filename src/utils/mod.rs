use serde::de::DeserializeOwned;
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

pub trait AsArray {
    fn as_array(&self) -> Vec<String>;
}
use csv::ReaderBuilder;
#[derive(Debug, serde::Deserialize, Clone)]
pub struct Record {
    pub role: String,
    pub tool: String,
    pub desc: String,
}
impl AsArray for Record {
    fn as_array(&self) -> Vec<String> {
        vec![self.role.clone(), self.tool.clone(), self.desc.clone()]
    }
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct HackingVariables {
    pub variable: String,
    pub path: String,
    pub category: String,
}
impl AsArray for HackingVariables {
    fn as_array(&self) -> Vec<String> {
        vec![
            self.variable.clone(),
            self.path.clone(),
            self.category.clone(),
        ]
    }
}

pub fn read_csv_data<T>(path: impl AsRef<Path>) -> Vec<T>
where
    T: DeserializeOwned,
    T: std::fmt::Debug,
{
    let mut csv_reader: csv::Reader<File> = ReaderBuilder::new()
        .delimiter(b';')
        .from_path(path)
        .expect("Error reading csv file");
    let rec_iter = csv_reader.deserialize();
    let records: Vec<T> = rec_iter
        .filter(|rec| rec.is_ok())
        .map(|rec| rec.unwrap())
        .collect();
    // println!("Records {:?}", records);
    return records;
}
