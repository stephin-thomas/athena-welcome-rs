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

#[derive(Debug, serde::Deserialize, Clone)]
pub struct ToolRecipe {
    pub tool: String,
    pub desc: String,
}
impl AsArray for ToolRecipe {
    fn as_array(&self) -> Vec<String> {
        vec![self.tool.clone(), self.desc.clone()]
    }
}
