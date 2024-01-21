use crate::APP_CONFIG_DIR;
use crate::SETTINGS_PATH;
use anyhow::{anyhow, Result};
use std::fs;
use std::fs::File;
use std::io::Write;
// Check if the config directory exist if not create them, Create and default config to settings file if it doesn't exist
pub fn init_settings() -> Result<()> {
    match &APP_CONFIG_DIR.try_exists() {
        Ok(res) => {
            if *res == true {
                // Create config Directory
                match fs::create_dir_all(&APP_CONFIG_DIR.as_path()) {
                    Err(err) => {
                        panic!("Error creating config directory:- {}", err);
                    }
                    Ok(()) => {
                        return Ok(());
                    }
                }
            }
        }
        Err(err) => {
            panic!("Determining if config dir exists failed with\n  {}", err)
        }
    };
    // Create a file
    match File::options()
        .write(true)
        .create_new(true)
        .open(SETTINGS_PATH.as_path())
    {
        Ok(mut settings_file) => {
            // Write contents to the file
            let _bytes_writtern = settings_file.write("autostart=True\nrole=none".as_bytes())?;
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
