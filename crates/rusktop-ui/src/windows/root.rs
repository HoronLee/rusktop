use gpui::*;
use gpui_component::Root;

use crate::views::WebServiceView;

pub fn create_root_window(window: &mut Window, cx: &mut Context<Root>) -> Root {
    let view = cx.new(|cx| WebServiceView::new(window, cx));
    Root::new(view, window, cx)
}

pub fn default_window_options(cx: &App) -> WindowOptions {
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
            None,
            size(px(400.0), px(300.0)),
            cx,
        ))),
        titlebar: Some(TitlebarOptions {
            title: Some("Rusktop - Web Service Control".into()),
            appears_transparent: false,
            ..Default::default()
        }),
        ..Default::default()
    }
}
