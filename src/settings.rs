use crate::APP_CONFIG_DIR;
use crate::SETTINGS_PATH;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    None,
    BlueTeamer,
    BugBountHunter,
    CrackerSpecialist,
    DoSTester,
    EnthusiastStudent,
    ForensicAnalyst,
    MalwareAnalyst,
    MobileAnalyst,
    NetworkAnalyst,
    OSINTSpecialist,
    RedTeamer,
    WebPentester,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::None => write!(f, "None"),
            Role::BlueTeamer => write!(f, "💙 Blue Teamer 💙"),
            Role::BugBountHunter => write!(f, "🐞 Bug Bounty Hunter 🐞"),
            Role::CrackerSpecialist => write!(f, "🍘 Cracker Specialist 🍘"),
            Role::DoSTester => write!(f, "💀 DoS Tester 💀"),
            Role::EnthusiastStudent => write!(f, "🎓 Enthusiast Student 🎓"),
            Role::ForensicAnalyst => write!(f, "🔍 Forensic Analyst 🔍"),
            Role::MalwareAnalyst => write!(f, "🦠 Malware Analyst 🦠"),
            Role::MobileAnalyst => write!(f, "📱 Mobile Analyst 📱"),
            Role::NetworkAnalyst => write!(f, "🖧 Network Analyst 🖧"),
            Role::OSINTSpecialist => write!(f, "🌐 OSINT Specialist 🌐"),
            Role::RedTeamer => write!(f, "❤️ Red Teamer ❤️",),
            Role::WebPentester => write!(f, "🕸️ Web Pentester 🕸️",),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    autostart: bool,
    role: Role, // Add other fields as needed
}

// Check if the config directory exist if not create them, Create and default config to settings file if it doesn't exist
pub fn init_settings() -> Result<()> {
    match APP_CONFIG_DIR.try_exists() {
        Ok(res) => {
            if res == false {
                // Create config Directory
                fs::create_dir_all(&APP_CONFIG_DIR.as_path())
                    .context("error creating config directories")?
            }
        }
        Err(err) => {
            return Err(anyhow!(
                "Determining if config dir exists failed with\n  {}",
                err
            ));
        }
    };
    // Create a file
    match File::options()
        .write(true)
        .create_new(true)
        .open(SETTINGS_PATH.as_path())
    {
        Ok(mut settings_file) => {
            let config = Config {
                autostart: true,
                role: Role::None,
            };

            // Write contents to the file
            let _bytes_writtern = settings_file.write(
                toml::to_string(&config)
                    .context("Error serialising default configs")?
                    .as_bytes(),
            )?;
            // let _bytes_writtern = settings_file.write("autostart=True\nrole=none".as_bytes())?;
            Ok(())
        }
        Err(err) => match err.kind() {
            std::io::ErrorKind::AlreadyExists => {
                println!("Existing settings file found");
                Ok(())
            }
            _ => return Err(anyhow!("Error saving settings file due to\n  {}", err)),
        },
    }
}

// Load settings from settings.conf file
pub fn load_settings() -> Result<Config> {
    let settings = toml::from_str(
        read_text_file(SETTINGS_PATH.as_path())
            .context("Error reading settings file. Unable to open")?
            .as_str(),
    )?;
    return Ok(settings);
}

// Read content of text file to string
pub(crate) fn read_text_file(file_name: impl AsRef<Path>) -> Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
