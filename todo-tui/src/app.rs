use color_eyre::{Result, eyre::WrapErr};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, widgets::ListState};

use crate::ui;

#[derive(Debug, Default, Clone, Copy)]
pub enum ApplicationMode {
    /// Editing to add new todo item
    Editing,
    /// Default to normal mode
    #[default]
    Normal,
}

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
    pub todos: Vec<Todo>,
    /// List state for todo
    pub todo_state: TodoState,
    /// Current mode
    pub input_mode: ApplicationMode,
    /// Input text from text area
    pub input_string: String,
}

#[derive(Debug, Default)]
pub struct TodoState {
    /// Tracks the list widget's viewing/selection state
    pub state: ListState,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> App {
        App::default()
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
        match self.input_mode {
            ApplicationMode::Normal => match key_event.code {
                KeyCode::Char('e') => self.input_mode = ApplicationMode::Editing,
                // TODO: Handling wrap around for the boundaries would be good
                KeyCode::Char('j') | KeyCode::Down => self.todo_state.state.select_next(),
                KeyCode::Char('k') | KeyCode::Up => self.todo_state.state.select_previous(),
                KeyCode::Char('q') | KeyCode::Esc => self.exit(),
                // Space will toggle a specific input that is selected to !completed
                KeyCode::Char(' ') => self.toggle_selected_item(),
                _ => {}
            },
            ApplicationMode::Editing => {
                match key_event.code {
                    // Escape the edit mode of input text area
                    // Does not clear the input text area
                    KeyCode::Esc => self.input_mode = ApplicationMode::Normal,
                    KeyCode::Enter => self.insert_todo(),
                    KeyCode::Backspace => self.delete_character(),
                    KeyCode::Char(x) => self.append_character(x),
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn toggle_selected_item(&mut self) {
        let Some(index) = self.todo_state.state.selected() else {
            return;
        };

        let Some(todo) = self.todos.get_mut(index) else {
            return;
        };

        todo.completed = !todo.completed;
    }

    fn exit(&mut self) {
        self.should_quit = true
    }

    fn insert_todo(&mut self) {
        // TODO: Empty handling of todo
        self.todos.push(Todo {
            item: self.input_string.clone(),
            completed: false,
        });
        // Clear input string
        self.input_string.clear();
        // Switch from edit to normal mode
        self.input_mode = ApplicationMode::Normal;
        // Select the newly added todo item
        self.todo_state.state.select_last();
    }

    fn delete_character(&mut self) {
        _ = self.input_string.pop();
    }

    fn append_character(&mut self, value: char) {
        // Store input string only append
        self.input_string.push(value);
    }
}
