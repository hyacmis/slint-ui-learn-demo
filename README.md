# Slint UI 学习计划

## 项目概述

这是一个个人学习 Slint 框架的仓库，采用"工作区+独立示例"的结构，通过多个小项目逐步掌握 Slint 的核心概念和高级功能。

## 学习进度

### 入门级 L1

- [x] 数据绑定（L1-calc）
  - 关键技术：全局模型（global）、双向绑定
- [ ] 基本元素
  - 关键技术：Image、Path、StyledText
- [ ] 基本控件（ComboBox、Slider、SpinBox、Switch、ProgressIndicator、Spinner）

### 基础级 L2

- [x] 条件渲染（L2-tab）
  - 关键技术：数据绑定、for 循环、条件渲染
- [x] 组件化（L2-sidebar）
  - 关键技术：自定义组件、状态管理、国际化（@tr）、Palette
- [ ] 手势处理（Flickable、ScaleRotateGestureHandler、SwipeGestureHandler）
- [ ] 窗口组件（Dialog、PopupWindow、MenuBar、ContextMenuArea）
- [ ] 布局组件（GridLayout、GridBox、GroupBox）
- [ ] 视图组件（StandardListView、StandardTableView）

### 中级 L3

- [x] 数据模型（L3-table）
  - 关键技术：Model、VecModel、ListView、结构体绑定
- [x] 复杂应用（L3-todo）
  - 关键技术：全局模型回调、业务逻辑分离
- [ ] Timer 定时器
- [ ] 动画系统（animate、transitions）
- [ ] 国际化（@tr 宏、翻译集成）
- [ ] 自定义样式（Palette、StyleMetrics）

### 高级 L4

- [x] 异步集成（L4-spawn_local）
  - 关键技术：spawn_local、async-compat、HTTP 请求
- [ ] 杂项组件（DatePickerPopup、TimePickerPopup、AboutSlint）
- [ ] 调试技术

## 项目结构

每个学习项目应包含：
```
project-name/
├── Cargo.toml      # 依赖配置
├── build.rs        # Slint 构建脚本
├── src/
│   └── main.rs     # Rust 代码
└── ui/
    └── app.slint   # UI 定义
```

## 参考资源

- [Slint 官方文档](https://slint.dev/docs.html)
- [Slint 标准组件库](https://slint.dev/latest/docs/slint/reference/overview/)
- [Slint 示例](https://github.com/slint-ui/slint/tree/master/examples)
- [SlintPad 在线编辑器](https://slintpad.com)
