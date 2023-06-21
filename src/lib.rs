/// Private macros
#[macro_use]
mod macros;
/// Main app
mod app;
/// 'Attempt' something, such as close a file
mod attempt;
/// Wrapper for `Sender` and `Receiver` types in `std::sync::mpsc`
mod channel;
/// Handle file input/output and save state
mod file;

mod export;

mod csv;

use std::path::PathBuf;

pub use crate::app::App;
use crate::{attempt::Attempt, channel::Channel, file::File};

#[macro_export]
macro_rules! print_info {
    ( $($tt:tt)* ) => {
        print!("[info] ");
        println!( $($tt)* );
    }
}

/// Cryption key which every file uses
///
/// This is not very secure, but at least the file cannot be opened by any program
const KEY: &str = "super-secure-encryption-key";

/// Set window scale
///
/// Affects window zoom, position, and size
pub const GLOBAL_WINDOW_SCALE: f32 = 0.6;

/// Get default directory to open file open/save dialogs in
fn get_start_dir() -> Option<PathBuf> {
    if let Some(dir) = dirs_next::document_dir() {
        return Some(dir);
    }
    if let Some(dir) = dirs_next::desktop_dir() {
        return Some(dir);
    }
    if let Some(dir) = dirs_next::home_dir() {
        return Some(dir);
    }
    None
}

/// Create simple file open/save dialog with `rfd`
fn file_dialog() -> rfd::FileDialog {
    file_dialog_no_filter().add_filter("Encrypted file", &["enc"])
}

/// Create simple file open/save dialog with `rfd`
///
/// Does not add a file filter
fn file_dialog_no_filter() -> rfd::FileDialog {
    let dialog = rfd::FileDialog::new();
    if let Some(dir) = get_start_dir() {
        dialog.set_directory(dir)
    } else {
        dialog
    }
}

/// Round a float to 2 decimal places and convert to string
fn round_to_string(number: f32) -> String {
    let rounded = (number * 100.0).round() / 100.0;
    rounded.to_string()
}
