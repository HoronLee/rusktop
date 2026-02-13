use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants},
    v_flex,
};
use rusktop_core::Counter;

pub struct CounterView {
    counter: Counter,
}

impl CounterView {
    pub fn new() -> Self {
        Self {
            counter: Counter::new(),
        }
    }
}

impl Render for CounterView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.counter.value();

        v_flex()
            .size_full()
            .justify_center()
            .items_center()
            .gap_4()
            .bg(rgb(0x1e1e1e))
            .child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .child(format!("计数: {}", count)),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        Button::new("decrement-btn")
                            .label("-")
                            .on_click(cx.listener(|view, _, _, _| {
                                view.counter.decrement();
                            })),
                    )
                    .child(Button::new("reset-btn").label("重置").on_click(cx.listener(
                        |view, _, _, _| {
                            view.counter.reset();
                        },
                    )))
                    .child(Button::new("increment-btn").primary().label("+").on_click(
                        cx.listener(|view, _, _, _| {
                            view.counter.increment();
                        }),
                    )),
            )
    }
}
