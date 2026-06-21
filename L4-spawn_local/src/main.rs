// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::ComponentHandle;
use std::error::Error;
slint::include_modules!();

async fn request_by_method() -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();

    let builder = client.get("https://httpbin.org/get");

    let response = builder.send().await?;

    Ok(response)
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let ui_weak = ui.as_weak();

    ui.on_send(move || {
        let handle = ui_weak.clone().unwrap();

        slint::spawn_local(async_compat::Compat::new(async move {
            let content = match request_by_method().await {
                Ok(resp) => resp.text().await.unwrap_or_else(|e| e.to_string()),
                Err(err) => err.to_string(),
            };

            // 更新 Slint UI
            handle.set_content(content.into());
        }))
        .unwrap();
    });

    ui.run()?;

    Ok(())
}
