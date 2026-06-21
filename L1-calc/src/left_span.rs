use crate::{AppWindow, CalcModel};
use slint::{CloseRequestResponse, ComponentHandle, Weak};
use std::fs;

pub fn startup(ui: Weak<AppWindow>) {
    // 启动时间
    // 窗口启动读取文件
    let num = fs::read_to_string("config.txt")
        .unwrap_or_else(|_| "0".to_string())
        .parse::<i32>()
        .unwrap_or(0);

    if let Some(ui) = ui.upgrade() {
        ui.global::<CalcModel>().set_num(num);
    }
}

pub fn shutdown(ui: Weak<AppWindow>) {
    // 窗口关闭事件
    let ui_weak = ui.clone();

    ui.unwrap().window().on_close_requested(move || {
        let num = ui_weak.unwrap().global::<CalcModel>().get_num();
        fs::write("config.txt", num.to_string()).unwrap();
        CloseRequestResponse::HideWindow
    })
}
