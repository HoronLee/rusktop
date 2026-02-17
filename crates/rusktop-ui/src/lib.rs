pub mod views;
pub mod windows;

pub use views::WebServiceView;
pub use windows::{create_root_window, default_window_options};

/// 启动 UI 应用
/// 
/// 这个函数封装了 GPUI 应用的初始化和运行逻辑，
/// 使得 rusktop-app 层不需要直接依赖 GPUI 框架。
pub fn run() {
    use gpui::*;
    
    let app = Application::new();
    
    app.run(|cx: &mut App| {
        gpui_component::init(cx);
        
        let window_options = default_window_options(cx);
        
        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| create_root_window(window, cx))
        })
        .expect("Failed to open window");
    });
}
