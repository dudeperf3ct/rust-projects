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

