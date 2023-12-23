use crate::structs::Todo;
use crate::utils;
use colorize::*;

pub fn add(title: String) {
    if title.len() < 1 { // Check if title is empty
        println!("{}", "No title provided".red());

        return;
    }

    let mut todos = utils::get_todos().unwrap(); // Get todos

    let todo = Todo {
        created_at: utils::get_timestamp(),
        title,
        done: false,
        id: utils::get_id(),
        updated_at: utils::get_timestamp(),
    };

    todos.push(todo); // Push todo to todos

    utils::save_todos(todos); // Save todos

    println!("{}", "Added todo".green());
}
pub fn list() {
    let todos = utils::get_todos().unwrap();

    if todos.len() == 0 {
        println!("{}", "No todos".red());
        return;
    }

    println!(
        "{0: <5} | {1: <20} | {2: <20} | {3: <20} | {4: <20}",
        "ID", "Title", "Created at", "Updated at", "Done"
    );

    println!();

    for todo in todos {
        println!(
            "{0: <5} | {1: <20} | {2: <20} | {3: <20} | {4: <20}",
            todo.id,
            todo.title,
            todo.created_at,
            todo.updated_at,
            if todo.done { "Completed ?".green() } else { "No ?".red() }
        );
    }
}

