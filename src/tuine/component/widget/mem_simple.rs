use std::cmp::{max, min};

use tui::{text::Text, widgets::Paragraph, Frame};

use crate::tuine::{Bounds, DrawContext, LayoutNode, Size, StateContext, TmpComponent};

/// A [`MemSimple`] is a widget displaying simple CPU stats.
pub struct MemSimple {}

impl super::AppWidget for MemSimple {
    fn build(
        ctx: &mut crate::tuine::BuildContext<'_>, painter: &crate::canvas::Painter,
        config: &crate::app::AppConfig, data: &mut crate::data_conversion::ConvertedData<'_>,
    ) -> Self {
        Self {}
    }
}

impl<Message> TmpComponent<Message> for MemSimple {
    fn draw<Backend>(
        &mut self, _state_ctx: &mut StateContext<'_>, draw_ctx: &DrawContext<'_>,
        frame: &mut Frame<'_, Backend>,
    ) where
        Backend: tui::backend::Backend,
    {
        let rect = draw_ctx.global_rect();
        frame.render_widget(
            Paragraph::new(Text::raw("Mem Simple")).block(tui::widgets::Block::default()),
            rect,
        );
    }

    fn layout(&self, bounds: Bounds, _node: &mut LayoutNode) -> Size {
        Size {
            width: bounds.max_width,
            height: max(bounds.min_height, min(2, bounds.max_height)),
        }
    }
}
