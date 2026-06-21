#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod todo;

use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let ui_weak = ui.as_weak();

    todo::update_todo(ui_weak);

    ui.run()?;

    Ok(())
}
