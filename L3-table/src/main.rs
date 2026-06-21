// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::{Model, ModelRc, VecModel};
use std::error::Error;
use std::rc::Rc;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let ui_weak = ui.as_weak();

    let rc_data = Rc::new(VecModel::<Person>::from(vec![]));
    let handle = ui_weak.unwrap();
    handle.set_data(ModelRc::from(rc_data.clone()));

    ui.on_add(move || {
        let data = rc_data.clone();
        let length = data.iter().len() as u8;
        data.push(Person {
            name: format!("name-{}", length + 1).into(),
            age: 0,
            job: format!("job-{}", length + 1).into(),
        })
    });

    ui.run()?;

    Ok(())
}
