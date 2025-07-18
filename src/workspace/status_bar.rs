use gpui::{Pixels, px};

use crate::{project::Project, theme::colors, ui::prelude::*};

pub struct StatusBar {
    project: Entity<Project>,
}

impl StatusBar {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn height(window: &mut Window) -> Pixels {
        (1.5 * window.rem_size()).max(px(34.0))
    }

    #[cfg(target_os = "windows")]
    pub fn height(_window: &mut Window) -> Pixels {
        // todo(windows) instead of hard coded size report the actual size to the Windows platform API
        px(32.0)
    }
}

impl Render for StatusBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let height = Self::height(window);

        div()
            .w_full()
            .h(height)
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .p_2()
            .border_t_1()
            .border_color(rgba(colors::BORDER))
            .text_sm()
            .child("status bar is wip".to_string())
            .child(
                IconButton::from_name("sort", IconName::Settings)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.project.update(cx, |project, cx| {
                            project.toggle_settings(cx);
                        });
                    }))
                    .tooltip(Tooltip::text("Toggle settings")),
            )
    }
}
