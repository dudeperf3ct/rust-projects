use color_eyre::{Result, eyre::WrapErr};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, widgets::ListState};

use crate::ui;

#[derive(Debug, Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    // Track the selection state for todo items
    // TODO: Is this required as part of App or could we move to separate UI state?
    pub todo_state: ListState,
    /// Container for storing todo items
    pub todo_items: Vec<String>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> App {
        App {
            should_quit: false,
            // TODO: start with no selection? First item selected? Last opened item selected?
            todo_state: ListState::default(),
            todo_items: vec![
                String::from("Item1"),
                String::from("Item2"),
                String::from("Item3"),
            ],
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| ui::render_frame(self, frame))?;
            self.handle_events().wrap_err("Handle events failed")?;
        }
        Ok(())
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            // TODO: Handling wrap around for the boundaries would be good
            KeyCode::Char('j') | KeyCode::Down => self.todo_state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.todo_state.select_previous(),
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.should_quit = true
    }
}
