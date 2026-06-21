use crate::{AppWindow, ImageInfo, ImageModel};
use slint::{ComponentHandle, Weak};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub fn bind(ui_weak: Weak<AppWindow>) {
    let current_path: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));
    let open_count: Rc<RefCell<i32>> = Rc::new(RefCell::new(0));
    let ui = ui_weak.unwrap();
    let model = ui.global::<ImageModel>();

    // 打开图片
    model.on_open_clicked({
        let ui_weak = ui_weak.clone();
        let path = current_path.clone();
        let count = open_count.clone();
        move || {
            let ui = ui_weak.unwrap();
            let model = ui.global::<ImageModel>();

            let file = rfd::FileDialog::new()
                .add_filter("图片", &["png", "jpg", "jpeg", "gif", "bmp", "webp", "svg"])
                .pick_file();

            if let Some(file_path) = file {
                match slint::Image::load_from_path(&file_path) {
                    Ok(image) => {
                        let sz = image.size();

                        // 更新 ViewModel
                        model.set_image_source(image);

                        let name = file_path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();

                        *count.borrow_mut() += 1;

                        model.set_info(ImageInfo {
                            file_name: name.into(),
                            file_path: file_path.to_string_lossy().to_string().into(),
                            img_width: sz.width as i32,
                            img_height: sz.height as i32,
                            open_count: *count.borrow(),
                        });

                        *path.borrow_mut() = Some(file_path);
                    }
                    Err(e) => {
                        eprintln!("加载图片失败: {}", e);
                    }
                }
            }
        }
    });

    // 另存为
    model.on_save_clicked({
        let path = current_path.clone();
        move || {
            let src = path.borrow().clone();
            let Some(src_path) = src else {
                eprintln!("请先打开一张图片");
                return;
            };

            let file = rfd::FileDialog::new()
                .add_filter("PNG", &["png"])
                .add_filter("JPEG", &["jpg", "jpeg"])
                .add_filter("BMP", &["bmp"])
                .add_filter("WebP", &["webp"])
                .save_file();

            if let Some(dest) = file
                && let Err(e) = std::fs::copy(&src_path, &dest)
            {
                eprintln!("保存失败: {}", e);
            }
        }
    });
}
