use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    // TODO: create database and navigator
    let db = JiraDatabase::new("./data/db.json".to_owned());

    let mut navigator = Navigator::new(Rc::new(db));

    loop {
        clearscreen::clear().unwrap();

        // TODO: implement the following functionality:

        // 1. get current page from navigator. If there is no current page exit the loop.
        let current_page = match navigator.get_current_page() {
            Some(page) => page,
            None => break,
        };
        // 2. render page
        if let Err(error) = current_page.draw_page() {
            println!(
                "Error rendering page: {}\nPress any key to continue...",
                error
            );
            wait_for_key_press();
        };
        // 3. get user input
        let input = get_user_input();
        // 4. pass input to page's input handler
        let action = current_page.handle_input(&input);
        // 5. if the page's input handler returns an action let the navigator process the action
        if let Err(error) = action {
            println!(
                "Error handling input: {}\nPress any key to continue...",
                error
            );
            wait_for_key_press();
            continue;
        }

        match action.unwrap() {
            Some(action) => {
                if let Err(error) = navigator.handle_action(action) {
                    println!(
                        "Error handling action: {}\nPress any key to continue...",
                        error
                    );
                    wait_for_key_press();
                    continue;
                }
            }
            None => {
                continue;
            }
        };
    }
}
