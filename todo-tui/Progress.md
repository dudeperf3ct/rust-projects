# Progress

Before we start, let's think about what the Todo application should be like

It should provide a place to type the todo, show the todo list, select unselect the todos, quit the application
More formally, the application state would cover
- List of todos
- Which todo is selected
- Editing mode? Edit mode, Select mode
- Current input text
- Completed/Not-completed status
- Quit the application

Let's think about the input iteraction
- Down key or j (vim-navigation style) means “move selection down”
- Enter means “toggle current todo” or “submit current input”
- e means "start editing"
- Esc means "cancel editing"
- q means "quit"

Let's incrementally build a TODO application starting with
1. Show a static list of todos in the terminal
2. Allow selection of todos
3. Allow marking todos as completed or toggling those
4. Allow inserting new todos (and editing/deleting existing ones)


## First and second phase

Here let's start with creating a simple TUI that shows static list of todos where user can navigate the list. Upon navigation, the element will be highlighted. 

We will break down this into 3 components

- App : What should the state contain that are useful for the application?
- UI : How should we display the application on terminal?
- Events : How should we interact with the application?

```rust
struct App {
    todos: Vec<String>,
    should_quit: bool,
    todo_state: ListState // uses List widget
}
```

UI

Shown in the picture

[List Widget](https://ratatui.rs/examples/widgets/list/) already can take us all the way there. It provides stateful widget that can track the selected item.

Events

- Ability to navigate the list of todos up and down using the arrow navigation keys
- Pressing q to quit the application

## Third phase

Here we move away from having only vector of string in todo, we also track whether the item is toggled as completed or not.

```rust
pub struct Todo {
    /// item in todo
    pub item: String,
    /// whether item is toggled as completed?
    pub completed: bool,
}

pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// container for todo with item and if it's completed or not
    pub todo: Vec<Todo>,
    /// List state for todo
    pub todo_state: TodoState,
}

pub struct TodoState {
    /// List state tracks list UI state such as selected row/scroll position. It does not store the todo items.
    pub state: ListState,
}
```

Now to render this in UI, we render the list as following using `completed` in the todo. Here 
*  ` [ ]` is shown along side list item if `completed` in todo is set to `false`
*  ` [x]` is shown along side list item if `completed` in todo is set to `true`
 
```rust
    let todo_list = List::from_iter(app.todo.iter().map(|todo| {
        let checkbox = if todo.completed { "[x]" } else { "[ ]" };

        ListItem::new(format!("{checkbox} {}", todo.item))
    }))
```

To change the state of the todo item, we add a event `KeyCode::Char(' ')`, whenever `Spacebar` is pressed along side the selected item, it is toggled to switch from `true` if set to `false` or `false` if value is already `true`.

```rust
fn toggle_selected_item(&mut self) {
        let selected_index = self.todo_state.state.selected().unwrap();
        let selected_todo: Option<&mut Todo> = self.todo.get_mut(selected_index);
        if let Some(todo) = selected_todo {
            todo.completed = !todo.completed
        }
    }
```

## Fourth phase

In the third phase, we were populating the list of item statically. In this phase, we want to have a separate input area which will send the input text as an item in the todo (dynamic).

So now we have two modes: non-editing mode and editing mode

In non-editing mode (normal):

* `j` / `Down` means move selection
* `k` / `Up` means move selection
* `Space` means toggle selected todo
* `q` means quit
* `e` means enter editing/input mode

Then when `e` is pressed, the meaning of keys changes.

In editing mode:

* normal letters should become input text
* `j` should probably mean the character "j", not move down
* `k` should probably mean the character "k", not move up
* `q` should probably mean the character "q", not quit
* `Esc` should stop editing

```rust
#[derive(Debug, Default)]
pub enum ApplicationMode {
    Edit,
    #[default]
    Normal,
}
```

Next is since we want to take input through a text area we need some way to keep track of it. For now we can have it as a simple `input_text` is `String` but it can get more complicated or it's own struct if we need cursor movement or input selection.

This introudces new methods that we need to take care of when in editing mode to handle `Backspace`, any character inserted and `Enter` pressed.

After this, we have to look into UI and how to render the text area. The layout changes with 4 elements like

```rust
let constraints = [
        Constraint::Length(1),
        Constraint::Min(1),
        Constraint::Percentage(10),
        Constraint::Length(1),
    ];
```

`Paragraph` widget is used to capture the input string and the text area is highlighted with different color depending on the application mode.

```rust
fn render_input_text(frame: &mut Frame, area: Rect, app: &mut App) {
    let input_text = Paragraph::new(app.input_string.join(""))
        .style(match app.input_mode {
            ApplicationMode::Normal => Style::default(),
            ApplicationMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::bordered().title("Input"));
    frame.render_widget(input_text, area);
}
```

Further

- [ ] Editing the todos
- [ ] Advanced cursor editing
- [ ] Displaying cleared todos maybe as strikethrough