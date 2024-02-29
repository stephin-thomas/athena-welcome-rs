use crate::APP_CONFIG_DIR;
use crate::SETTINGS_PATH;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
use strum_macros::EnumIter;
use toml;

#[derive(Debug, Serialize, Deserialize, EnumIter, Clone)]
pub enum Role {
    None,
    BlueTeamer,
    BugBountyHunter,
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
impl Role {
    pub fn id(&self) -> &'static str {
        match self {
            Role::None => "none",
            Role::BlueTeamer => "blue",
            Role::BugBountyHunter => "bugbounty",
            Role::CrackerSpecialist => "cracker",
            Role::DoSTester => "dos",
            Role::EnthusiastStudent => "student",
            Role::ForensicAnalyst => "forensic",
            Role::MalwareAnalyst => "malware",
            Role::MobileAnalyst => "mobile",
            Role::NetworkAnalyst => "network",
            Role::OSINTSpecialist => "osint",
            Role::RedTeamer => "red",
            Role::WebPentester => "web",
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Role::None => "None",
            Role::BlueTeamer => "Blue Teamer",
            Role::BugBountyHunter => "Bug Bounty Hunter",
            Role::CrackerSpecialist => "Cracker Specialist",
            Role::DoSTester => "DoS Tester",
            Role::EnthusiastStudent => "Enthusiast Student",
            Role::ForensicAnalyst => "Forensic Analyst",
            Role::MalwareAnalyst => "Malware Analyst",
            Role::MobileAnalyst => "Mobile Analyst",
            Role::NetworkAnalyst => "Network Analyst",
            Role::OSINTSpecialist => "OSINT Specialist",
            Role::RedTeamer => "Red Teamer",
            Role::WebPentester => "Web Pentester",
        }
    }
}
impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::None => write!(f, "🔥 Choose your Role 🔥"),
            Role::BlueTeamer => write!(f, "💙 Blue Teamer 💙"),
            Role::BugBountyHunter => write!(f, "🐞 Bug Bounty Hunter 🐞"),
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub autostart: bool,
    pub role: Role, // Add other fields as needed
}

impl Config {
    // Create and save default config to settings file if it doesn't exist
    pub fn init() -> Result<()> {
        match APP_CONFIG_DIR.try_exists() {
            Ok(res) => {
                if !res {
                    // Create config Directory
                    fs::create_dir_all(APP_CONFIG_DIR.as_path())
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
            Ok(_) => {
                let config = Config {
                    autostart: true,
                    role: Role::None,
                };

                config.save()?;
                // let _bytes_writtern = settings_file.write("autostart=True\nrole=none".as_bytes())?;
                Ok(())
            }
            Err(err) => match err.kind() {
                std::io::ErrorKind::AlreadyExists => {
                    println!("Existing settings file found");
                    Ok(())
                }
                _ => Err(anyhow!("Error saving settings file due to\n  {}", err)),
            },
        }
    }

    // Load configs from settings.conf file
    pub fn load() -> Result<Config> {
        let config = toml::from_str(
            read_text_file(SETTINGS_PATH.as_path())
                .context("Error reading settings file. Unable to open")?
                .as_str(),
        )?;
        Ok(config)
    }

    // Save the config
    pub fn save(&self) -> Result<(), anyhow::Error> {
        let mut settings_file = File::options()
            .truncate(true)
            .write(true)
            .open(SETTINGS_PATH.as_path())
            .unwrap();
        let _bytes_written = settings_file.write(
            toml::to_string(&self)
                .context("Error serialising default configs")?
                .as_bytes(),
        )?;
        println!("Config Updated {:?}", &self);
        Ok(())
    }
}

// Read content of text file to string
pub(crate) fn read_text_file(file_name: impl AsRef<Path>) -> Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
