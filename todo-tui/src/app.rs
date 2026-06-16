use color_eyre::{Result, eyre::WrapErr};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, widgets::ListState};

use crate::ui;

#[derive(Debug, Default)]
pub struct Todo {
    /// item in todo
    pub item: String,
    /// whether item is toggled as completed?
    pub completed: bool,
}

#[derive(Debug, Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// container for todo with item and if it's completed or not
    pub todo: Vec<Todo>,
    /// List state for todo
    pub todo_state: TodoState,
}

#[derive(Debug, Default)]
pub struct TodoState {
    /// List state is a stateful widget that keeps track of selected item and rest of items in list
    pub state: ListState,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> App {
        App {
            todo: vec![Todo {
                item: String::from("this is a item"),
                completed: false,
            }],
            ..Default::default()
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
            KeyCode::Char('j') | KeyCode::Down => self.todo_state.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.todo_state.state.select_previous(),
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            // Space will toggle a specific input that is selected to !completed
            KeyCode::Char(' ') => self.toggle_selected_item(),
            _ => {}
        }
        Ok(())
    }

    fn toggle_selected_item(&mut self) {
        let selected_index = self.todo_state.state.selected().unwrap();
        let selected_todo: Option<&mut Todo> = self.todo.get_mut(selected_index);
        if let Some(todo) = selected_todo {
            todo.completed = !todo.completed
        }
    }

    fn exit(&mut self) {
        self.should_quit = true
    }
}
