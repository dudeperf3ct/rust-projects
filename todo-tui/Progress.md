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
- Down key means “move selection down”
- Enter means “toggle current todo” or “submit current input”
- i means "start editing"
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
    /// List state is a stateful widget that keeps track of selected item and rest of items in list
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