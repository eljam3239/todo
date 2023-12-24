use crate::todo::*;
use crate::utils;
use colorize::*;

pub fn start(){
    utils::init();
    let args = utils::get_args();
    match args.command.as_str(){
        "add" => add(args.arguments),
        "list" => list(),
        "done" => done(args.arguments),
        "remove" => remove(args.arguments),
        "q" => std::process::exit(0),
        _ => {
                println!("{}", "            No command found - Showing help".black());

                let help = format!(
                    "
                {} {}
                {}
                -----
        
                Help:
        
                Command   | Arguments | Description
                {}           text      Add a new todo
                {}                    List all todos
                {}           id       Mark a todo as done
                {}         id       Delete a todo
            ",
                    "Welcome to".grey(),
                    "Todo".cyan(),
                    "Simple todo app written in Rust".black(),
                    "add".cyan(),
                    "list".blue(),
                    "done".green(),
                    "remove".red()
                );

                println!("{help}");
        }
}

}


