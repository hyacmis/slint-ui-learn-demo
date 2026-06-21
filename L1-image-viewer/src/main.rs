#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod image_handler;

use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    image_handler::bind(ui.as_weak());

    ui.run()?;
    Ok(())
}
