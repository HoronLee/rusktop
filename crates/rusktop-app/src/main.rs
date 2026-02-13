use gpui::*;
use rusktop_ui::CounterView;

fn main() {
    let app = Application::new();

    app.run(|cx: &mut App| {
        gpui_component::init(cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(400.0), px(300.0)),
                cx,
            ))),
            titlebar: Some(TitlebarOptions {
                title: Some("Rusktop - 计数器示例".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |_, cx| cx.new(|_| CounterView::new()))
            .expect("Failed to open window");
    });
}
