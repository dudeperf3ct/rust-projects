use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{List, ListItem},
};

use crate::app::App;

pub fn render_frame(app: &mut App, frame: &mut Frame) {
    let constraints = [
        Constraint::Length(1),
        Constraint::Min(1),
        Constraint::Length(1),
    ];
    let layout = Layout::vertical(constraints).spacing(1);
    let [top, todo_area, bottom] = frame.area().layout(&layout);

    render_header(frame, top, app);
    render_list(frame, todo_area, app);
    render_footer(frame, bottom, app);
}

fn render_header(frame: &mut Frame, area: Rect, app: &mut App) {
    let top_title = Line::from("TODO Items").bold();
    frame.render_widget(top_title.centered(), area);
}

fn render_footer(frame: &mut Frame, area: Rect, app: &mut App) {
    let bottom_title = Line::from(vec![
        Span::styled("↑/↓", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Navigate  "),
        Span::styled("q", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Quit"),
    ]);
    frame.render_widget(bottom_title.centered(), area);
}

fn render_list(frame: &mut Frame, area: Rect, app: &mut App) {
    let todo_list = List::from_iter(app.todo.iter().map(|todo| {
        let checkbox = if todo.completed { "[x]" } else { "[ ]" };

        ListItem::new(format!("{checkbox} {}", todo.item))
    }))
    .highlight_symbol("> ")
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    frame.render_stateful_widget(todo_list, area, &mut app.todo_state.state);
}
