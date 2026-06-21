#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod left_span;

use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let ui_weak = ui.as_weak();

    left_span::startup(ui_weak.clone());

    left_span::shutdown(ui_weak.clone());

    ui.run()?;

    Ok(())
}
