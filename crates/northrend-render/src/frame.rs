use crate::{Color, RenderView};

#[derive(Debug, Clone, Copy)]
pub struct RenderFrame<'a> {
    pub clear_color: Color,
    pub views: &'a [RenderView<'a>],
}

impl<'a> RenderFrame<'a> {
    pub const fn new(
        clear_color: Color,
        views: &'a [RenderView<'a>],
    ) -> Self {
        Self {
            clear_color,
            views,
        }
    }
}
