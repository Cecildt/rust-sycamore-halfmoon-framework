use sycamore::prelude::*;
use uuid::Uuid;

use super::{todo::Todo, filter::Filter};

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub todos: RcSignal<Vec<RcSignal<Todo>>>,
    pub filter: RcSignal<Filter>,
}

impl AppState {
    pub fn add_todo(&self, title: String) {
        self.todos.set(
            self.todos
                .get()
                .as_ref()
                .clone()
                .into_iter()
                .chain(Some(create_rc_signal(Todo {
                    title,
                    completed: false,
                    id: Uuid::new_v4(),
                })))
                .collect(),
        )
    }

    pub fn remove_todo(&self, id: Uuid) {
        self.todos.set(
            self.todos
                .get()
                .iter()
                .filter(|todo| todo.get().id != id)
                .cloned()
                .collect(),
        );
    }

    pub fn todos_left(&self) -> usize {
        self.todos.get().iter().fold(
            0,
            |acc, todo| if todo.get().completed { acc } else { acc + 1 },
        )
    }

    pub fn toggle_complete_all(&self) {
        if self.todos_left() == 0 {
            // make all todos active
            for todo in self.todos.get().iter() {
                if todo.get().completed {
                    todo.set(Todo {
                        completed: false,
                        ..todo.get().as_ref().clone()
                    })
                }
            }
        } else {
            // make all todos completed
            for todo in self.todos.get().iter() {
                if !todo.get().completed {
                    todo.set(Todo {
                        completed: true,
                        ..todo.get().as_ref().clone()
                    })
                }
            }
        }
    }

    pub fn clear_completed(&self) {
        self.todos.set(
            self.todos
                .get()
                .iter()
                .filter(|todo| !todo.get().completed)
                .cloned()
                .collect(),
        );
    }
}
