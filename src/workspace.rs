use gpui::{
    App, Bounds, Context, CursorStyle, Decorations, Div, Entity, Global, Hsla, IntoElement,
    MouseButton, Pixels, Point, ResizeEdge, Size, Stateful, Tiling, Window, canvas, div, point,
    prelude::*, px, rgb, size, transparent_black,
};
use main_pane::MainPane;
use status_bar::StatusBar;
use title_bar::TitleBar;

use crate::theme::{self, colours};

mod main_pane;
mod status_bar;
mod title_bar;

// pub struct State {}

pub struct Workspace {
    main_pane: Entity<MainPane>,
    status_bar: Entity<StatusBar>,
    title_bar: Entity<TitleBar>,
}

impl Workspace {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            main_pane: cx.new(|_| MainPane {}),
            status_bar: cx.new(|_| StatusBar {}),
            title_bar: cx.new(|_| TitleBar::new()),
        }
    }
}

// based on zed workspace
impl Render for Workspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        client_side_decorations(
            div()
                // .key_context(context)
                .relative()
                .size_full()
                .flex()
                .flex_col()
                // .font(ui_font)
                .gap_0()
                .justify_start()
                .items_start()
                .text_color(rgb(colours::TEXT))
                .overflow_hidden()
                .child(self.title_bar.clone())
                .child(
                    div()
                        .id("workspace")
                        .bg(rgb(colours::BACKGROUND))
                        .relative()
                        .flex_1()
                        .w_full()
                        .flex()
                        .flex_col()
                        .overflow_hidden()
                        .border_t_1()
                        .border_b_1()
                        .border_color(rgb(colours::BORDER))
                        .child(self.main_pane.clone())
                        .child(self.status_bar.clone()),
                ),
            window,
            cx,
        )
    }
}

// copied from zed workspace
pub fn client_side_decorations(
    element: impl IntoElement,
    window: &mut Window,
    _cx: &mut App,
) -> Stateful<Div> {
    const BORDER_SIZE: Pixels = px(1.0);
    let decorations = window.window_decorations();

    if matches!(decorations, Decorations::Client { .. }) {
        window.set_client_inset(theme::CLIENT_SIDE_DECORATION_SHADOW);
    }

    struct GlobalResizeEdge(ResizeEdge);
    impl Global for GlobalResizeEdge {}

    div()
        .id("window-backdrop")
        .bg(transparent_black())
        .map(|div| match decorations {
            Decorations::Server => div,
            Decorations::Client { tiling, .. } => div
                .when(!(tiling.top || tiling.right), |div| {
                    div.rounded_tr(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                })
                .when(!(tiling.top || tiling.left), |div| {
                    div.rounded_tl(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                })
                .when(!(tiling.bottom || tiling.right), |div| {
                    div.rounded_br(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                })
                .when(!(tiling.bottom || tiling.left), |div| {
                    div.rounded_bl(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                })
                .when(!tiling.top, |div| {
                    div.pt(theme::CLIENT_SIDE_DECORATION_SHADOW)
                })
                .when(!tiling.bottom, |div| {
                    div.pb(theme::CLIENT_SIDE_DECORATION_SHADOW)
                })
                .when(!tiling.left, |div| {
                    div.pl(theme::CLIENT_SIDE_DECORATION_SHADOW)
                })
                .when(!tiling.right, |div| {
                    div.pr(theme::CLIENT_SIDE_DECORATION_SHADOW)
                })
                .on_mouse_move(move |e, window, cx| {
                    let size = window.window_bounds().get_bounds().size;
                    let pos = e.position;

                    let new_edge =
                        resize_edge(pos, theme::CLIENT_SIDE_DECORATION_SHADOW, size, tiling);

                    let edge = cx.try_global::<GlobalResizeEdge>();
                    if new_edge != edge.map(|edge| edge.0) {
                        window
                            .window_handle()
                            .update(cx, |workspace, _, cx| {
                                cx.notify(workspace.entity_id());
                            })
                            .ok();
                    }
                })
                .on_mouse_down(MouseButton::Left, move |e, window, _| {
                    let size = window.window_bounds().get_bounds().size;
                    let pos = e.position;

                    let edge = match resize_edge(
                        pos,
                        theme::CLIENT_SIDE_DECORATION_SHADOW,
                        size,
                        tiling,
                    ) {
                        Some(value) => value,
                        None => return,
                    };

                    window.start_window_resize(edge);
                }),
        })
        .size_full()
        .child(
            div()
                .cursor(CursorStyle::Arrow)
                .map(|div| match decorations {
                    Decorations::Server => div,
                    Decorations::Client { tiling } => div
                        .border_color(rgb(colours::BORDER))
                        .when(!(tiling.top || tiling.right), |div| {
                            div.rounded_tr(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                        })
                        .when(!(tiling.top || tiling.left), |div| {
                            div.rounded_tl(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                        })
                        .when(!(tiling.bottom || tiling.right), |div| {
                            div.rounded_br(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                        })
                        .when(!(tiling.bottom || tiling.left), |div| {
                            div.rounded_bl(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                        })
                        .when(!tiling.top, |div| div.border_t(BORDER_SIZE))
                        .when(!tiling.bottom, |div| div.border_b(BORDER_SIZE))
                        .when(!tiling.left, |div| div.border_l(BORDER_SIZE))
                        .when(!tiling.right, |div| div.border_r(BORDER_SIZE))
                        .when(!tiling.is_tiled(), |div| {
                            div.shadow(smallvec::smallvec![gpui::BoxShadow {
                                color: Hsla {
                                    h: 0.,
                                    s: 0.,
                                    l: 0.,
                                    a: 0.4,
                                },
                                blur_radius: theme::CLIENT_SIDE_DECORATION_SHADOW / 2.,
                                spread_radius: px(0.),
                                offset: point(px(0.0), px(0.0)),
                            }])
                        }),
                })
                .on_mouse_move(|_e, _, cx| {
                    cx.stop_propagation();
                })
                .size_full()
                .child(element),
        )
        .map(|div| match decorations {
            Decorations::Server => div,
            Decorations::Client { tiling, .. } => div.child(
                canvas(
                    |_bounds, window, _| {
                        window.insert_hitbox(
                            Bounds::new(
                                point(px(0.0), px(0.0)),
                                window.window_bounds().get_bounds().size,
                            ),
                            false,
                        )
                    },
                    move |_bounds, hitbox, window, cx| {
                        let mouse = window.mouse_position();
                        let size = window.window_bounds().get_bounds().size;
                        let Some(edge) =
                            resize_edge(mouse, theme::CLIENT_SIDE_DECORATION_SHADOW, size, tiling)
                        else {
                            return;
                        };
                        cx.set_global(GlobalResizeEdge(edge));
                        window.set_cursor_style(
                            match edge {
                                ResizeEdge::Top | ResizeEdge::Bottom => CursorStyle::ResizeUpDown,
                                ResizeEdge::Left | ResizeEdge::Right => {
                                    CursorStyle::ResizeLeftRight
                                }
                                ResizeEdge::TopLeft | ResizeEdge::BottomRight => {
                                    CursorStyle::ResizeUpLeftDownRight
                                }
                                ResizeEdge::TopRight | ResizeEdge::BottomLeft => {
                                    CursorStyle::ResizeUpRightDownLeft
                                }
                            },
                            &hitbox,
                        );
                    },
                )
                .size_full()
                .absolute(),
            ),
        })
}

// copied from zed workspace
fn resize_edge(
    pos: Point<Pixels>,
    shadow_size: Pixels,
    window_size: Size<Pixels>,
    tiling: Tiling,
) -> Option<ResizeEdge> {
    let bounds = Bounds::new(Point::default(), window_size).inset(shadow_size * 1.5);
    if bounds.contains(&pos) {
        return None;
    }

    let corner_size = size(shadow_size * 1.5, shadow_size * 1.5);
    let top_left_bounds = Bounds::new(Point::new(px(0.), px(0.)), corner_size);
    if !tiling.top && top_left_bounds.contains(&pos) {
        return Some(ResizeEdge::TopLeft);
    }

    let top_right_bounds = Bounds::new(
        Point::new(window_size.width - corner_size.width, px(0.)),
        corner_size,
    );
    if !tiling.top && top_right_bounds.contains(&pos) {
        return Some(ResizeEdge::TopRight);
    }

    let bottom_left_bounds = Bounds::new(
        Point::new(px(0.), window_size.height - corner_size.height),
        corner_size,
    );
    if !tiling.bottom && bottom_left_bounds.contains(&pos) {
        return Some(ResizeEdge::BottomLeft);
    }

    let bottom_right_bounds = Bounds::new(
        Point::new(
            window_size.width - corner_size.width,
            window_size.height - corner_size.height,
        ),
        corner_size,
    );
    if !tiling.bottom && bottom_right_bounds.contains(&pos) {
        return Some(ResizeEdge::BottomRight);
    }

    if !tiling.top && pos.y < shadow_size {
        Some(ResizeEdge::Top)
    } else if !tiling.bottom && pos.y > window_size.height - shadow_size {
        Some(ResizeEdge::Bottom)
    } else if !tiling.left && pos.x < shadow_size {
        Some(ResizeEdge::Left)
    } else if !tiling.right && pos.x > window_size.width - shadow_size {
        Some(ResizeEdge::Right)
    } else {
        None
    }
}
