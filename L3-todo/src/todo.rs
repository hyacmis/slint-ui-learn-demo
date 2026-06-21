use crate::{AppWindow, TodoItem, TodoModel};
use slint::{ComponentHandle, Model, ModelRc, VecModel, Weak};
use std::rc::Rc;

pub fn update_todo(ui_weak: Weak<AppWindow>) {
    // 创建初始的空 todo 列表
    let items = Rc::new(VecModel::<TodoItem>::from(vec![]));
    let ui = ui_weak.unwrap();

    let todo = ui.global::<TodoModel>();
    todo.set_todo_model(ModelRc::from(items.clone()));

    let items_clone = items.clone();
    todo.on_add(move |item| {
        items_clone.push(item);
    });

    let items_clone2 = items.clone();

    todo.on_remove(move || {
        let idx_list: Vec<usize> = items
            .iter()
            .enumerate()
            .filter(|(_, item)| item.checked)
            .map(|(idx, _)| idx)
            .collect();

        // 从后向前删除，防止出现下标越界的问题
        for index in idx_list.iter().rev() {
            items_clone2.remove(*index);
        }
    })
}
