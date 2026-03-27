mod layout;
mod timer_view;
mod schedule_bar;
mod controls;

use ratatui::Frame;
use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let areas = layout::build_layout(frame.area(), app);
    timer_view::render(frame, app, areas.timer);
    schedule_bar::render(frame, app, areas.schedule);
    controls::render(frame, app, areas.controls);
}
