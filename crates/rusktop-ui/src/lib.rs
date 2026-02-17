pub mod views;
pub mod windows;

pub use views::WebServiceView;
pub use windows::{create_root_window, default_window_options};

use gpui::Global;

#[derive(Clone)]
pub struct GlobalAppConfig(pub rusktop_core::AppConfig);

impl Global for GlobalAppConfig {}

impl std::ops::Deref for GlobalAppConfig {
    type Target = rusktop_core::AppConfig;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// 启动 UI 应用
/// 
/// 这个函数封装了 GPUI 应用的初始化和运行逻辑，
/// 使得 rusktop-app 层不需要直接依赖 GPUI 框架。
pub fn run(config: rusktop_core::AppConfig) {
    use gpui::*;
    
    let app = Application::new();
    
    app.run(move |cx: &mut App| {
        // 将配置注册为全局状态，供所有视图访问
        cx.set_global(GlobalAppConfig(config));
        
        gpui_component::init(cx);
        
        let window_options = default_window_options(cx);
        
        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| create_root_window(window, cx))
        })
        .expect("Failed to open window");
    });
}
