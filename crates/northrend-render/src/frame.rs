use crate::{Color, RenderView};

#[derive(Debug, Clone, Copy)]
pub struct RenderFrame<'a, M, T> {
    pub clear_color: Color,
    pub views: &'a [RenderView<'a, M, T>],
}

impl<'a, M, T> RenderFrame<'a, M, T> {
    pub const fn new(
        clear_color: Color,
        views: &'a [RenderView<'a, M, T>],
    ) -> Self {
        Self {
            clear_color,
            views,
        }
    }
}
