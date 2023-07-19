use crate::{
    geometry::size_requirements::WidgetSizeRequirement,
    drawing::canvas::Canvas, app::event::input_event::InputEvent
};


pub(crate) mod align;
pub(crate) mod background;
pub(crate) mod button;
pub(crate) mod center;
pub(crate) mod column;
pub(crate) mod empty;
pub(crate) mod expanded;
pub(crate) mod padder;
pub(crate) mod row;
pub(crate) mod scaffold;
pub(crate) mod sized_box;

/// The widget trait. All widgets are stored as Box<dyn Widget>.
/// This trait can be used to create custom widgets, that can be implemented from scratch or use a combination of existing widgets.
pub trait Widget {
    /// Draw the widget on the canvas. The given rect is the area the widget should draw in, computed by its parent
    /// with it's size requirements. 
    fn draw(&self, canvas: &mut Canvas, rect: softbuffer::Rect);
    /// Get the size requirements of this widget.
    /// If the widgets requests sized outside of the constraints, they will be given smaller sizes to be drawn in.
    fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement);
    /// Handles an event. Returns true if the event was handled, false otherwise.
    /// This will be called on the root, and need to be propagated down the widget for each custom widget implementation, 
    /// 
    /// It is needed to recompute the widgets rect while doing so: events are called one after another, and there is no guarantee
    /// that draw will be called between each event. As events can change widget layouts, it is needed to recompute the rect
    /// to ensure that the next event is handled correctly. 
    fn handle_event(&mut self, event: InputEvent, rect: softbuffer::Rect) -> bool;
}