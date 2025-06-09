use clap::{crate_description, crate_name, crate_version};
use std::sync::{Mutex, OnceLock};

use crate::sys;

// APP information
pub const CRATE_BIN_NAME: &str = "fsutil";
pub const CRATE_UPDATE_DATE: &str = "2025-06-09";

/// Global Mutex lock guard for quiet mode
pub static QUIET_MODE: OnceLock<Mutex<bool>> = OnceLock::new();

/// Check if quiet mode is enabled
pub fn is_quiet_mode() -> bool {
    match QUIET_MODE.get() {
        Some(mutex) => match mutex.try_lock() {
            Ok(guard) => *guard,
            Err(_) => false,
        },
        None => false,
    }
}

pub fn set_quiet_mode(enabled: bool) -> Result<(), String> {
    let mutex: &Mutex<bool> = QUIET_MODE.get_or_init(|| Mutex::new(false));
    match mutex.try_lock() {
        Ok(mut guard) => {
            *guard = enabled;
            Ok(())
        }
        Err(_) => Err("Failed to lock mutex".to_string()),
    }
}

pub enum AppCommands {
    Copy,
    Move,
    Delete,
    List,
    Batch,
}

impl AppCommands {
    pub fn from_str(s: &str) -> Option<AppCommands> {
        match s {
            "copy" => Some(AppCommands::Copy),
            "move" => Some(AppCommands::Move),
            "delete" => Some(AppCommands::Delete),
            "list" => Some(AppCommands::List),
            "batch" => Some(AppCommands::Batch),
            _ => None,
        }
    }
}

pub fn show_app_desc() {
    if is_quiet_mode() {
        return;
    }
    println!(
        "{} v{} ({}) {}",
        crate_name!(),
        crate_version!(),
        CRATE_UPDATE_DATE,
        sys::os::get_os_type()
    );
    println!("{}", crate_description!());
    println!();
    println!("'{} --help' for more information.", CRATE_BIN_NAME);
    println!();
}

pub fn show_banner_with_starttime() {
    if is_quiet_mode() {
        return;
    }
    println!(
        "{} v{} {}",
        crate_name!(),
        crate_version!(),
        sys::os::get_os_type()
    );
    println!();
    println!("Starting at {}", sys::time::get_sysdate());
    println!();
}

pub fn exit_with_error_message(message: &str) {
    println!();
    println!("Error: {}", message);
    std::process::exit(1);
}

pub fn show_error_with_help(message: &str) {
    println!();
    println!("Error: {}", message);
    println!();
    println!("'{} --help' for more information.", CRATE_BIN_NAME);
    println!();
}
