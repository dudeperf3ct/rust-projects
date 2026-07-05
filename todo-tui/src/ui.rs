use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Paragraph},
};

use crate::app::{App, ApplicationMode};

pub fn render_frame(app: &mut App, frame: &mut Frame) {
    let constraints = [
        Constraint::Length(1),
        Constraint::Min(1),
        Constraint::Percentage(10),
        Constraint::Length(1),
    ];
    let layout = Layout::vertical(constraints).spacing(1);
    let [top, todo_area, text_area, bottom] = frame.area().layout(&layout);

    render_header(frame, top);
    render_list(frame, todo_area, app);
    render_input_text(frame, text_area, app);
    render_footer(frame, bottom);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let top_title = Line::from("Todo Items").bold();
    frame.render_widget(top_title.centered(), area);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let bottom_title = Line::from(vec![
        Span::styled("↑/↓", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Navigate  "),
        Span::styled("Space", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Toggle  "),
        Span::styled("e", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Edit  "),
        Span::styled("Esc", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Stop editing  "),
        Span::styled("q", Style::default().fg(Color::Cyan).bold()),
        Span::raw(" Quit"),
    ]);
    frame.render_widget(bottom_title.centered(), area);
}

fn render_list(frame: &mut Frame, area: Rect, app: &mut App) {
    let todo_list = List::from_iter(app.todos.iter().map(|todo| {
        let checkbox = if todo.completed { "[x]" } else { "[ ]" };

        ListItem::new(format!("{checkbox} {}", todo.item))
    }))
    .highlight_symbol("> ")
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    frame.render_stateful_widget(todo_list, area, &mut app.todo_state.state);
}

fn render_input_text(frame: &mut Frame, area: Rect, app: &mut App) {
    let input_text = Paragraph::new(app.input_string.clone())
        .style(match app.input_mode {
            ApplicationMode::Normal => Style::default(),
            ApplicationMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::bordered().title("Input"));
    frame.render_widget(input_text, area);
}
