use crate::{theme::colors, ui::prelude::*};

pub struct Tooltip {
    title: SharedString,
}

impl Tooltip {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
        }
    }

    pub fn text(title: impl Into<SharedString>) -> impl Fn(&mut Window, &mut App) -> AnyView {
        let title = title.into();
        move |_, cx| cx.new(|_| Self::new(title.clone())).into()
    }
}

impl Render for Tooltip {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        tooltip_container(window, cx, |this, _, _| {
            this.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_4()
                    .child(div().max_w_72().child(self.title.clone())),
            )
        })
    }
}

pub fn tooltip_container<V>(
    window: &mut Window,
    cx: &mut Context<V>,
    f: impl FnOnce(Div, &mut Window, &mut Context<V>) -> Div,
) -> impl IntoElement {
    // padding to avoid tooltip appearing right below the mouse cursor
    div().pl_2().pt_2p5().child(
        div()
            .flex()
            .flex_col()
            .items_center()
            // .elevation_2(cx)
            .bg(rgba(colors::ELEVATED_SURFACE_BACKGROUND))
            .rounded_lg()
            .border_1()
            .border_color(rgba(colors::BORDER_VARIANT))
            .shadow_md()
            // .font(ui_font)
            // .text_ui(cx)
            .text_sm()
            .text_color(rgba(colors::TEXT))
            .py_1()
            .px_2()
            .map(|el| f(el, window, cx)),
    )
}
